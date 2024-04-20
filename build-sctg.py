#!/usr/bin/env python3

import os
import glob
import pathlib
import platform
import zipfile
import urllib.request
import shutil
import hashlib
import argparse
import re
import sys

windows = platform.platform().startswith('Windows')
osx = platform.platform().startswith(
    'Darwin') or platform.platform().startswith("macOS")
hbb_name = 'rustdesk' + ('.exe' if windows else '')
exe_path = 'target/release/' + hbb_name
if windows:
    flutter_build_dir = 'build/windows/x64/runner/Release/'
elif osx:
    flutter_build_dir = 'build/macos/Build/Products/Release/'
else:
    flutter_build_dir = 'build/linux/x64/release/bundle/'
flutter_build_dir_2 = f'flutter/{flutter_build_dir}'
skip_cargo = False


def get_arch() -> str:
    custom_arch = os.environ.get("ARCH")
    if custom_arch is None:
        return "amd64"
    return custom_arch


def system2(cmd):
    err = os.system(cmd)
    if err != 0:
        print(f"Error occurred when executing: {cmd}. Exiting.")
        sys.exit(-1)


def get_version():
    with open("Cargo.toml", encoding="utf-8") as fh:
        for line in fh:
            if line.startswith("version"):
                return line.replace("version", "").replace("=", "").replace('"', '').strip()
    return ''


def parse_rc_features(feature):
    available_features = {
        'PrivacyMode': {
            'platform': ['windows'],
            'zip_url': 'https://github.com/fufesou/RustDeskTempTopMostWindow/releases/download/v0.3'
                       '/TempTopMostWindow_x64.zip',
            'checksum_url': 'https://github.com/fufesou/RustDeskTempTopMostWindow/releases/download/v0.3/checksum_md5',
            'include': ['WindowInjection.dll'],
        }
    }
    apply_features = {}
    if not feature:
        feature = []

    def platform_check(platforms):
        if windows:
            return 'windows' in platforms
        elif osx:
            return 'osx' in platforms
        else:
            return 'linux' in platforms

    def get_all_features():
        features = []
        for (feat, feat_info) in available_features.items():
            if platform_check(feat_info['platform']):
                features.append(feat)
        return features

    if isinstance(feature, str) and feature.upper() == 'ALL':
        return get_all_features()
    elif isinstance(feature, list):
        if windows:
            # download third party is deprecated, we use github ci instead.
            # force add PrivacyMode
            # feature.append('PrivacyMode')
            pass
        for feat in feature:
            if isinstance(feat, str) and feat.upper() == 'ALL':
                return get_all_features()
            if feat in available_features:
                if platform_check(available_features[feat]['platform']):
                    apply_features[feat] = available_features[feat]
            else:
                print(f'Unrecognized feature {feat}')
        return apply_features
    else:
        raise Exception(f'Unsupported features param {feature}')


def make_parser():
    parser = argparse.ArgumentParser(description='Build script.')
    parser.add_argument(
        '-f',
        '--feature',
        dest='feature',
        metavar='N',
        type=str,
        nargs='+',
        default='',
        help='Integrate features, windows only.'
             'Available: PrivacyMode. Special value is "ALL" and empty "". Default is empty.')
    parser.add_argument('--flutter', action='store_true',
                        help='Build flutter package', default=False)
    parser.add_argument('--debug', action='store_true',
                        help='Build flutter package with debug flag', default=False)
    parser.add_argument(
        '--quic',
        action='store_true',
        help='Enable feature Quic'
    )
    parser.add_argument(
        '--vram',
        action='store_true',
        help='Enable feature vram, only available on windows now.'
    )
    parser.add_argument(
        '--hwcodec',
        action='store_true',
        help='Enable feature hwcodec' + (
            '' if windows or osx else ', need libva-dev, libvdpau-dev.')
    )
    parser.add_argument(
        '--gpucodec',
        action='store_true',
        help='Enable feature gpucodec, only available on windows now.'
    )
    parser.add_argument(
        '--portable',
        action='store_true',
        help='Build windows portable'
    )
    parser.add_argument(
        '--unix-file-copy-paste',
        action='store_true',
        help='Build with unix file copy paste feature'
    )
    parser.add_argument(
        '--flatpak',
        action='store_true',
        help='Build rustdesk libs with the flatpak feature enabled'
    )
    parser.add_argument(
        '--appimage',
        action='store_true',
        help='Build rustdesk libs with the appimage feature enabled'
    )
    parser.add_argument(
        '--skip-cargo',
        action='store_true',
        help='Skip cargo build process, only flutter version + Linux supported currently'
    )
    if windows:
        parser.add_argument(
            '--skip-portable-pack',
            action='store_true',
            help='Skip packing, only flutter version + Windows supported'
        )
    parser.add_argument(
        '--ios',
        action='store_true',
        help='Build ios ipa'
    )
    parser.add_argument(
        '--web',
        action='store_true',
        help='Build web app'
    )
    parser.add_argument(
        "--package",
        type=str
    )
    return parser


# Generate build script for docker
#
# it assumes all build dependencies are installed in environments
# Note: do not use it in bare metal, or may break build environments
def generate_build_script_for_docker():
    with open("/tmp/build.sh", "w") as f:
        f.write('''
            #!/bin/bash
            # environment
            export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation: " | cut -d' ' -f4-)/include"
            # flutter
            pushd /opt
            wget https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_3.0.5-stable.tar.xz
            tar -xvf flutter_linux_3.0.5-stable.tar.xz
            export PATH=`pwd`/flutter/bin:$PATH
            popd
            # flutter_rust_bridge
            dart pub global activate ffigen --version 5.0.1
            pushd /tmp && git clone https://github.com/SoLongAndThanksForAllThePizza/flutter_rust_bridge --depth=1 && popd
            pushd /tmp/flutter_rust_bridge/frb_codegen && cargo install --path . && popd
            pushd flutter && flutter pub get && popd
            ~/.cargo/bin/flutter_rust_bridge_codegen --rust-input ./src/flutter_ffi.rs --dart-output ./flutter/lib/generated_bridge.dart
            # install vcpkg
            pushd /opt
            export VCPKG_ROOT=`pwd`/vcpkg
            git clone https://github.com/microsoft/vcpkg
            vcpkg/bootstrap-vcpkg.sh
            popd
            $VCPKG_ROOT/vcpkg install --x-install-root="$VCPKG_ROOT/installed"
            # build rustdesk
            ./build.py --flutter --hwcodec
        ''')
    system2("chmod +x /tmp/build.sh")
    system2("bash /tmp/build.sh")


# Downloading third party resources is deprecated.
# We can use this function in an offline build environment.
# Even in an online environment, we recommend building third-party resources yourself.
def download_extract_features(features, res_dir):
    import re

    proxy = ''

    def req(url):
        if not proxy:
            return url
        else:
            r = urllib.request.Request(url)
            r.set_proxy(proxy, 'http')
            r.set_proxy(proxy, 'https')
            return r

    for (feat, feat_info) in features.items():
        includes = feat_info['include'] if 'include' in feat_info and feat_info['include'] else []
        includes = [re.compile(p) for p in includes]
        excludes = feat_info['exclude'] if 'exclude' in feat_info and feat_info['exclude'] else []
        excludes = [re.compile(p) for p in excludes]

        print(f'{feat} download begin')
        download_filename = feat_info['zip_url'].split('/')[-1]
        checksum_md5_response = urllib.request.urlopen(
            req(feat_info['checksum_url']))
        for line in checksum_md5_response.read().decode('utf-8').splitlines():
            if line.split()[1] == download_filename:
                checksum_md5 = line.split()[0]
                filename, _headers = urllib.request.urlretrieve(feat_info['zip_url'],
                                                                download_filename)
                md5 = hashlib.md5(open(filename, 'rb').read()).hexdigest()
                if checksum_md5 != md5:
                    raise Exception(f'{feat} download failed')
                print(f'{feat} download end. extract bein')
                zip_file = zipfile.ZipFile(filename)
                zip_list = zip_file.namelist()
                for f in zip_list:
                    file_exclude = False
                    for p in excludes:
                        if p.match(f) is not None:
                            file_exclude = True
                            break
                    if file_exclude:
                        continue

                    file_include = False if includes else True
                    for p in includes:
                        if p.match(f) is not None:
                            file_include = True
                            break
                    if file_include:
                        print(f'extract file {f}')
                        zip_file.extract(f, res_dir)
                zip_file.close()
                os.remove(download_filename)
                print(f'{feat} extract end')


def external_resources(flutter, args, res_dir):
    features = parse_rc_features(args.feature)
    if not features:
        return

    print(f'Build with features {list(features.keys())}')
    if os.path.isdir(res_dir) and not os.path.islink(res_dir):
        shutil.rmtree(res_dir)
    elif os.path.exists(res_dir):
        raise Exception(f'Find file {res_dir}, not a directory')
    os.makedirs(res_dir, exist_ok=True)
    download_extract_features(features, res_dir)
    if flutter:
        os.makedirs(flutter_build_dir_2, exist_ok=True)
        for f in pathlib.Path(res_dir).iterdir():
            print(f'{f}')
            if f.is_file():
                shutil.copy2(f, flutter_build_dir_2)
            else:
                shutil.copytree(f, f'{flutter_build_dir_2}{f.stem}')


def get_features(args):
    features = ['inline'] if not args.flutter else []
    if args.quic:
        features.append('quic')
    if args.hwcodec:
        features.append('hwcodec')
    if args.vram:
        features.append('vram')
    if args.gpucodec:
        features.append('gpucodec')
    if args.flutter:
        features.append('flutter')
        features.append('flutter_texture_render')
    if args.flatpak:
        features.append('flatpak')
    if args.appimage:
        features.append('appimage')
    if args.unix_file_copy_paste:
        features.append('unix-file-copy-paste')
    print("features:", features)
    return features


def generate_control_file(version):
    control_file_path = "../res/DEBIAN/control"
    system2('/bin/rm -rf %s' % control_file_path)

    content = """Package: rustdesk
Version: %s
Architecture: %s
Maintainer: rustdesk <info@rustdesk.com>
Homepage: https://rustdesk.com
Depends: libgtk-3-0, libxcb-randr0, libxdo3, libxfixes3, libxcb-shape0, libxcb-xfixes0, libasound2, libsystemd0, curl, libva-drm2, libva-x11-2, libvdpau1, libgstreamer-plugins-base1.0-0, libpam0g, libappindicator3-1, gstreamer1.0-pipewire
Description: A remote control software.

""" % (version, get_arch())
    file = open(control_file_path, "w")
    file.write(content)
    file.close()


def ffi_bindgen_function_refactor():
    # workaround ffigen
    system2(
        'sed -i "s/ffi.NativeFunction<ffi.Bool Function(DartPort/ffi.NativeFunction<ffi.Uint8 Function(DartPort/g" flutter/lib/generated_bridge.dart')


def build_flutter_deb(version, features):
    if not skip_cargo:
        system2(f'cargo build --features {features} --lib --release')
        ffi_bindgen_function_refactor()
    os.chdir('flutter')
    system2('flutter build linux --release')
    system2('mkdir -p tmpdeb/usr/bin/')
    system2('mkdir -p tmpdeb/usr/lib/rustdesk')
    system2('mkdir -p tmpdeb/etc/rustdesk/')
    system2('mkdir -p tmpdeb/etc/pam.d/')
    system2('mkdir -p tmpdeb/usr/share/rustdesk/files/systemd/')
    system2('mkdir -p tmpdeb/usr/share/icons/hicolor/256x256/apps/')
    system2('mkdir -p tmpdeb/usr/share/icons/hicolor/scalable/apps/')
    system2('mkdir -p tmpdeb/usr/share/applications/')
    system2('mkdir -p tmpdeb/usr/share/polkit-1/actions')
    system2('rm tmpdeb/usr/bin/rustdesk || true')
    system2(
        f'cp -r {flutter_build_dir}/* tmpdeb/usr/lib/rustdesk/')
    system2(
        'cp ../res/rustdesk.service tmpdeb/usr/share/rustdesk/files/systemd/')
    system2(
        'cp ../res/128x128@2x.png tmpdeb/usr/share/icons/hicolor/256x256/apps/rustdesk.png')
    system2(
        'cp ../res/scalable.svg tmpdeb/usr/share/icons/hicolor/scalable/apps/rustdesk.svg')
    system2(
        'cp ../res/rustdesk.desktop tmpdeb/usr/share/applications/rustdesk.desktop')
    system2(
        'cp ../res/rustdesk-link.desktop tmpdeb/usr/share/applications/rustdesk-link.desktop')
    system2(
        'cp ../res/com.rustdesk.RustDesk.policy tmpdeb/usr/share/polkit-1/actions/')
    system2(
        'cp ../res/startwm.sh tmpdeb/etc/rustdesk/')
    system2(
        'cp ../res/xorg.conf tmpdeb/etc/rustdesk/')
    system2(
        'cp ../res/pam.d/rustdesk.debian tmpdeb/etc/pam.d/rustdesk')
    system2(
        "echo \"#!/bin/sh\" >> tmpdeb/usr/share/rustdesk/files/polkit && chmod a+x tmpdeb/usr/share/rustdesk/files/polkit")

    system2('mkdir -p tmpdeb/DEBIAN')
    generate_control_file(version)
    system2('cp -a ../res/DEBIAN/* tmpdeb/DEBIAN/')
    md5_file('usr/share/rustdesk/files/systemd/rustdesk.service')
    system2('dpkg-deb -b tmpdeb rustdesk.deb;')

    system2('/bin/rm -rf tmpdeb/')
    system2('/bin/rm -rf ../res/DEBIAN/control')
    os.rename('rustdesk.deb', '../rustdesk-%s.deb' % version)
    os.chdir("..")


def build_deb_from_folder(version, binary_folder):
    os.chdir('flutter')
    system2('mkdir -p tmpdeb/usr/bin/')
    system2('mkdir -p tmpdeb/usr/lib/rustdesk')
    system2('mkdir -p tmpdeb/usr/share/rustdesk/files/systemd/')
    system2('mkdir -p tmpdeb/usr/share/icons/hicolor/256x256/apps/')
    system2('mkdir -p tmpdeb/usr/share/icons/hicolor/scalable/apps/')
    system2('mkdir -p tmpdeb/usr/share/applications/')
    system2('mkdir -p tmpdeb/usr/share/polkit-1/actions')
    system2('rm tmpdeb/usr/bin/rustdesk || true')
    system2(
        f'cp -r ../{binary_folder}/* tmpdeb/usr/lib/rustdesk/')
    system2(
        'cp ../res/rustdesk.service tmpdeb/usr/share/rustdesk/files/systemd/')
    system2(
        'cp ../res/128x128@2x.png tmpdeb/usr/share/icons/hicolor/256x256/apps/rustdesk.png')
    system2(
        'cp ../res/scalable.svg tmpdeb/usr/share/icons/hicolor/scalable/apps/rustdesk.svg')
    system2(
        'cp ../res/rustdesk.desktop tmpdeb/usr/share/applications/rustdesk.desktop')
    system2(
        'cp ../res/rustdesk-link.desktop tmpdeb/usr/share/applications/rustdesk-link.desktop')
    system2(
        'cp ../res/com.rustdesk.RustDesk.policy tmpdeb/usr/share/polkit-1/actions/')
    system2(
        "echo \"#!/bin/sh\" >> tmpdeb/usr/share/rustdesk/files/polkit && chmod a+x tmpdeb/usr/share/rustdesk/files/polkit")

    system2('mkdir -p tmpdeb/DEBIAN')
    generate_control_file(version)
    system2('cp -a ../res/DEBIAN/* tmpdeb/DEBIAN/')
    md5_file('usr/share/rustdesk/files/systemd/rustdesk.service')
    system2('dpkg-deb -b tmpdeb rustdesk.deb;')

    system2('/bin/rm -rf tmpdeb/')
    system2('/bin/rm -rf ../res/DEBIAN/control')
    os.rename('rustdesk.deb', '../rustdesk-%s.deb' % version)
    os.chdir("..")

def replace_in_file(file_path, old_string, new_string):
    with open(file_path, 'r', encoding='utf-8', errors='replace') as file:
        file_data = file.read()

    # Remplacer la chaîne
    file_data = file_data.replace(old_string, new_string)

    # Écrire le résultat dans le fichier
    with open(file_path, 'w', encoding='utf-8') as file:
        file.write(file_data)

def replace_in_all_dart_files(old_string, new_string):
    for file_path in glob.glob('**/*.dart', recursive=True):
        replace_in_file(file_path, old_string, new_string)

def replace_in_all_rust_files(old_string, new_string):
    for file_path in glob.glob('**/*.rs', recursive=True):
        replace_in_file(file_path, old_string, new_string)

def replace_in_all_toml_files(old_string, new_string):
    for file_path in glob.glob('**/*.toml', recursive=True):
        replace_in_file(file_path, old_string, new_string)

def replace_in_all_typed_files(type, old_string, new_string):
    for file_path in glob.glob(f'**/*.{type}', recursive=True):
        replace_in_file(file_path, old_string, new_string)

def insert_line_after(file_path, search_string, new_line):
    with open(file_path, 'r') as file:
        lines = file.readlines()

    for i, line in enumerate(lines):
        if search_string in line:
            # Check if the next line is already equal to new_line
            if i + 1 < len(lines) and lines[i + 1].strip() == new_line:
                break
            else:
                lines.insert(i + 1, new_line + '\n')
                break

    with open(file_path, 'w') as file:
        file.writelines(lines)

def sctgdesk_customization():
    MACOS_CODESIGN_IDENTITY = os.environ.get('MACOS_CODESIGN_IDENTITY')
    APP_NAME = os.environ['APP_NAME']
    RENDEZVOUS_SERVER = os.environ['RENDEZVOUS_SERVER']
    RS_PUB_KEY = os.environ['RS_PUB_KEY']
    API_SERVER = os.environ['API_SERVER']
    ORG_NAME = os.environ['ORG_NAME']
    ORG_NAME_CAPITALIZED = ORG_NAME.capitalize()
    TEAM_ID="HZF9JMC8YN"
    match = re.search(r'\(([A-Z0-9]+)\)$', MACOS_CODESIGN_IDENTITY)
    if match:
        TEAM_ID = match.group(1)
    replace_in_file('flutter/ios/Runner.xcodeproj/project.pbxproj',"HZF9JMC8YN",TEAM_ID)
    replace_in_file('flutter/web/client.html', '__API_SERVER__', f'https://{API_SERVER}')
    replace_in_file('libs/hbb_common/src/config.rs', 'rs-ny.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_file('libs/hbb_common/src/config.rs', 'OeVuKk5nlHiXp+APNn0Y3pC1Iwpwn44JGqrQCsWqmBw=', RS_PUB_KEY)
    replace_in_file('flutter/web/js/src/connection.ts', 'const PORT = 21116', 'const PORT = 21118')
    replace_in_file('flutter/web/js/src/connection.ts', 'uri = tmp[0] + ":" + (port + (isRelay ? roffset || 3 : 2));','uri = tmp[0] + ":" + (PORT + (isRelay ? 3 : 2));')
    replace_in_file('flutter/web/js/src/connection.ts', 'ws://', 'wss://')
    replace_in_file('libs/portable/generate.py', 'rustdesk.exe', f'.{APP_NAME}.exe'.lower())
    replace_in_file('flatpak/rustdesk.json', 'rustdesk-', f'{APP_NAME}-'.lower())
    replace_in_file('flutter/lib/desktop/pages/desktop_setting_page.dart', 'https://rustdesk.com', f'https://{ORG_NAME}.eu.org'.lower())
    replace_in_file('flutter/lib/desktop/pages/desktop_setting_page.dart', 'Purslane Ltd.', f'{ORG_NAME_CAPITALIZED} and Purslane Ltd.')
    replace_in_file('flutter/lib/mobile/pages/settings_page.dart', 'https://rustdesk.com', f'https://{ORG_NAME}.eu.org'.lower())
    replace_in_file('flutter/lib/mobile/pages/settings_page.dart', 'Purslane Ltd.', f'{ORG_NAME_CAPITALIZED} and Purslane Ltd.')
    replace_in_all_typed_files('ts', 'OeVuKk5nlHiXp+APNn0Y3pC1Iwpwn44JGqrQCsWqmBw=', RS_PUB_KEY)
    replace_in_all_typed_files('ts', 'rs-ny.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_all_typed_files('ts', 'rs-sg.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_all_typed_files('ts', 'rs-cn.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_all_typed_files('ts', 'rs-us.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_all_typed_files('js', 'OeVuKk5nlHiXp+APNn0Y3pC1Iwpwn44JGqrQCsWqmBw=', RS_PUB_KEY)
    replace_in_all_typed_files('js', 'rs-ny.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_all_typed_files('js', 'rs-sg.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_all_typed_files('js', 'rs-cn.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_all_typed_files('js', 'rs-us.rustdesk.com', RENDEZVOUS_SERVER)
    replace_in_all_typed_files('rc', 'RustDesk', f'{APP_NAME}')
    replace_in_all_typed_files('rc', 'rustdesk', f'{APP_NAME}'.lower())
    replace_in_file('src/common.rs', 'https://admin.rustdesk.com', f'https://{API_SERVER}')
    replace_in_all_dart_files('RustDesk', f'{APP_NAME}')
    replace_in_all_rust_files('RustDesk', f'{APP_NAME}')
    replace_in_all_toml_files('RustDesk', f'{APP_NAME}')
    replace_in_all_toml_files('"rustdesk"', f'"{APP_NAME}"'.lower())
    replace_in_all_toml_files('"rustdesk.exe"', f'"{APP_NAME}"'.lower())
    replace_in_all_typed_files('rc','"rustdesk.exe"', f'"{APP_NAME}"'.lower())
    replace_in_all_typed_files('plist', 'RustDesk', f'{APP_NAME}')
    replace_in_all_typed_files('plist', 'com.carriez.rustdesk', f'com.{ORG_NAME}.{APP_NAME}'.lower())
    replace_in_all_typed_files('plist', 'com.carriez.flutterHbb', f'com.{ORG_NAME}.{APP_NAME}-ios'.lower())
    replace_in_all_typed_files('pbxproj', 'com.carriez.flutterHbb', f'com.{ORG_NAME}.{APP_NAME}-ios'.lower())
    replace_in_all_typed_files('html', 'rustdesk', f'{APP_NAME}'.lower())
    replace_in_all_typed_files('xcscheme', 'RustDesk', f'{APP_NAME}')
    replace_in_all_typed_files('xcconfig', 'RustDesk', f'{APP_NAME}')
    replace_in_all_typed_files('xcconfig', 'com.carriez', f'com.{ORG_NAME}'.lower())
    replace_in_all_typed_files('desktop', 'RustDesk', f'{APP_NAME}')
    replace_in_all_typed_files('service', 'RustDesk', f'{APP_NAME}')
    insert_line_after('src/common.rs','pub fn get_api_server',f'    return format!("https://{API_SERVER}");')
    insert_line_after('flutter/web/js/src/globals.js','function getApiServer()',f'  return "https://{API_SERVER}";')

def build_ios_ipa(version, features):
    MACOS_CODESIGN_IDENTITY = os.environ.get('MACOS_CODESIGN_IDENTITY')
    IOS_BUNDLE_ID = os.environ.get('IOS_BUNDLE_ID')
    IOS_DISTRIBUTION_PROFILE = os.environ.get('IOS_DISTRIBUTION_PROFILE')
    if not skip_cargo:
        system2(f'cargo build --features flutter --release --target aarch64-apple-ios --lib')
    os.chdir('flutter')
    system2('flutter build ipa --release --no-codesign')
    os.chdir('..')

def build_flutter_dmg(version, features, args):
    if not skip_cargo:
        # set minimum osx build target, now is 10.14, which is the same as the flutter xcode project
        if not args.debug:
            system2(
                f'MACOSX_DEPLOYMENT_TARGET=10.14 cargo build --features {features} --lib --release')
            system2(
                "cp target/release/liblibrustdesk.dylib target/release/librustdesk.dylib")
        else:
            system2(
                f'MACOSX_DEPLOYMENT_TARGET=10.14 cargo build --features {features} --lib')
            system2(
                "cp target/debug/liblibrustdesk.dylib target/debug/librustdesk.dylib")

    os.chdir('flutter')
    if not args.debug:
        system2('flutter build macos --release')
    else:
        system2('flutter build macos -v --debug')
    APP_NAME = os.environ['APP_NAME']
    app_name = APP_NAME.lower()
    MACOS_CODESIGN_IDENTITY = os.environ.get('MACOS_CODESIGN_IDENTITY')
    system2(f'codesign --force --options runtime -s "{MACOS_CODESIGN_IDENTITY}" --deep --strict ./build/macos/Build/Products/Release/{APP_NAME}.app -vvv')
    system2(
        f"create-dmg --volname \"RustDesk Installer\" --window-pos 200 120 --window-size 800 400 --icon-size 100 --app-drop-link 600 185 --icon {APP_NAME}.app 200 190 --hide-extension {APP_NAME}.app {app_name}.dmg ./build/macos/Build/Products/Release/{APP_NAME}.app")
    os.rename(f"{app_name}.dmg", f"../{app_name}-{version}.dmg")
    system2(f'codesign --force --options runtime -s "{MACOS_CODESIGN_IDENTITY}" --deep --strict ../{app_name}-{version}.dmg -vvv')
    os.chdir("..")

def build_web_app():
    os.chdir('flutter')
    os.chdir('web/js')
    system2('npx vite build')
    os.chdir('../..')
    system2('flutter pub upgrade')
    system2('flutter build web --release --source-maps')
    os.chdir("..")

def build_flutter_arch_manjaro(version, features):
    if not skip_cargo:
        system2(f'cargo build --features {features} --lib --release')
    ffi_bindgen_function_refactor()
    os.chdir('flutter')
    system2('flutter build linux --release')
    system2(f'strip {flutter_build_dir}/lib/librustdesk.so')
    os.chdir('../res')
    system2('HBB=`pwd`/.. FLUTTER=1 makepkg -f')


def build_flutter_windows(version, features, skip_portable_pack, args):
    build_mode = 'debug' if args.debug else 'release'
    if not skip_cargo:
        if not args.debug:
            system2(f'cargo build --features {features} --lib --release')
        else:
            system2(f'cargo build --features {features} --lib')
        if not os.path.exists(f"target/{build_mode}/librustdesk.dll"):
            print("cargo build failed, please check rust source code.")
            exit(-1)
    os.chdir('flutter')
    system2('flutter pub upgrade')
    if args.debug:
        system2('flutter build windows --debug')
    else:
        system2('flutter build windows --release')
    os.chdir('..')
    shutil.copy2(f'target/{build_mode}/deps/dylib_virtual_display.dll',
                 flutter_build_dir_2)
    if skip_portable_pack:
        return
    os.chdir('libs/portable')
    system2('pip3 install -r requirements.txt')
    system2(
        f'python3 ./generate.py -f ../../{flutter_build_dir_2} -o . -e ../../{flutter_build_dir_2}/rustdesk.exe')
    os.chdir('../..')
    if os.path.exists('./rustdesk_portable.exe'):
        os.replace(f'./target/{build_mode}/rustdesk-portable-packer.exe',
                   './rustdesk_portable.exe')
    else:
        os.rename(f'./target/{build_mode}/rustdesk-portable-packer.exe',
                  './rustdesk_portable.exe')
    print(
        f'output location: {os.path.abspath(os.curdir)}/rustdesk_portable.exe')
    os.rename('./rustdesk_portable.exe', f'./rustdesk-{version}-install.exe')
    print(
        f'output location: {os.path.abspath(os.curdir)}/rustdesk-{version}-install.exe')


def main():
    global skip_cargo
    parser = make_parser()
    args = parser.parse_args()

    if os.path.exists(exe_path):
        os.unlink(exe_path)
    if os.path.isfile('/usr/bin/pacman'):
        system2('git checkout src/ui/common.tis')
    version = get_version()
    features = ','.join(get_features(args))
    flutter = args.flutter
    ios = args.ios
    web = args.web
    if not flutter:
        system2('python3 res/inline-sciter.py')
    sctgdesk_customization()
    print(args.skip_cargo)
    if args.skip_cargo:
        skip_cargo = True
    portable = args.portable
    package = args.package
    if package:
        build_deb_from_folder(version, package)
        return
    res_dir = 'resources'
    external_resources(flutter, args, res_dir)
    if web:
        build_web_app()
        return
    if windows:
        # build virtual display dynamic library
        os.chdir('libs/virtual_display/dylib')
        system2('cargo build --release')
        os.chdir('../../..')

        if flutter:
            build_flutter_windows(version, features, args.skip_portable_pack, args)
            return
        system2('cargo build --release --features ' + features)
        # system2('upx.exe target/release/rustdesk.exe')
        system2('mv target/release/rustdesk.exe target/release/RustDesk.exe')
        pa = os.environ.get('P')
        if pa:
            # https://certera.com/kb/tutorial-guide-for-safenet-authentication-client-for-code-signing/
            system2(
                f'signtool sign /a /v /p {pa} /debug /f .\\cert.pfx /t http://timestamp.digicert.com  '
                'target\\release\\rustdesk.exe')
        else:
            print('Not signed')
        system2(
            f'cp -rf target/release/RustDesk.exe {res_dir}')
        os.chdir('libs/portable')
        system2('pip3 install -r requirements.txt')
        system2(
            f'python3 ./generate.py -f ../../{res_dir} -o . -e ../../{res_dir}/rustdesk-{version}-win7-install.exe')
        system2('mv ../../{res_dir}/rustdesk-{version}-win7-install.exe ../..')
    elif os.path.isfile('/usr/bin/pacman'):
        # pacman -S -needed base-devel
        system2("sed -i 's/pkgver=.*/pkgver=%s/g' res/PKGBUILD" % version)
        if flutter:
            build_flutter_arch_manjaro(version, features)
        else:
            system2('cargo build --release --features ' + features)
            system2('git checkout src/ui/common.tis')
            system2('strip target/release/rustdesk')
            system2('ln -s res/pacman_install && ln -s res/PKGBUILD')
            system2('HBB=`pwd` makepkg -f')
        system2('mv rustdesk-%s-0-x86_64.pkg.tar.zst rustdesk-%s-manjaro-arch.pkg.tar.zst' % (
            version, version))
        # pacman -U ./rustdesk.pkg.tar.zst
    elif os.path.isfile('/usr/bin/yum'):
        system2('cargo build --release --features ' + features)
        system2('strip target/release/rustdesk')
        system2(
            "sed -i 's/Version:    .*/Version:    %s/g' res/rpm.spec" % version)
        system2('HBB=`pwd` rpmbuild -ba res/rpm.spec')
        system2(
            'mv $HOME/rpmbuild/RPMS/x86_64/rustdesk-%s-0.x86_64.rpm ./rustdesk-%s-fedora28-centos8.rpm' % (
                version, version))
        # yum localinstall rustdesk.rpm
    elif os.path.isfile('/usr/bin/zypper'):
        system2('cargo build --release --features ' + features)
        system2('strip target/release/rustdesk')
        system2(
            "sed -i 's/Version:    .*/Version:    %s/g' res/rpm-suse.spec" % version)
        system2('HBB=`pwd` rpmbuild -ba res/rpm-suse.spec')
        system2(
            'mv $HOME/rpmbuild/RPMS/x86_64/rustdesk-%s-0.x86_64.rpm ./rustdesk-%s-suse.rpm' % (
                version, version))
        # yum localinstall rustdesk.rpm
    else:
        if flutter:
            if ios:
                build_ios_ipa(version, features)
            elif osx:
                build_flutter_dmg(version, features, args)
                pass
            else:
                # system2(
                #     'mv target/release/bundle/deb/rustdesk*.deb ./flutter/rustdesk.deb')
                build_flutter_deb(version, features)
        else:
            system2('cargo bundle --release --features ' + features)
            if osx:
                system2(
                    'strip target/release/bundle/osx/RustDesk.app/Contents/MacOS/rustdesk')
                system2(
                    'cp libsciter.dylib target/release/bundle/osx/RustDesk.app/Contents/MacOS/')
                # https://github.com/sindresorhus/create-dmg
                system2('/bin/rm -rf *.dmg')
                pa = os.environ.get('P')
                if pa:
                    system2('''
    # buggy: rcodesign sign ... path/*, have to sign one by one
    # install rcodesign via cargo install apple-codesign
    #rcodesign sign --p12-file ~/.p12/rustdesk-developer-id.p12 --p12-password-file ~/.p12/.cert-pass --code-signature-flags runtime ./target/release/bundle/osx/RustDesk.app/Contents/MacOS/rustdesk
    #rcodesign sign --p12-file ~/.p12/rustdesk-developer-id.p12 --p12-password-file ~/.p12/.cert-pass --code-signature-flags runtime ./target/release/bundle/osx/RustDesk.app/Contents/MacOS/libsciter.dylib
    #rcodesign sign --p12-file ~/.p12/rustdesk-developer-id.p12 --p12-password-file ~/.p12/.cert-pass --code-signature-flags runtime ./target/release/bundle/osx/RustDesk.app
    # goto "Keychain Access" -> "My Certificates" for below id which starts with "Developer ID Application:"
    codesign -s "Developer ID Application: {0}" --force --options runtime  ./target/release/bundle/osx/RustDesk.app/Contents/MacOS/*
    codesign -s "Developer ID Application: {0}" --force --options runtime  ./target/release/bundle/osx/RustDesk.app
    '''.format(pa))
                system2(
                    'create-dmg "RustDesk %s.dmg" "target/release/bundle/osx/RustDesk.app"' % version)
                os.rename('RustDesk %s.dmg' %
                          version, 'rustdesk-%s.dmg' % version)
                if pa:
                    system2('''
    # https://pyoxidizer.readthedocs.io/en/apple-codesign-0.14.0/apple_codesign.html
    # https://pyoxidizer.readthedocs.io/en/stable/tugger_code_signing.html
    # https://developer.apple.com/developer-id/
    # goto xcode and login with apple id, manager certificates (Developer ID Application and/or Developer ID Installer) online there (only download and double click (install) cer file can not export p12 because no private key)
    #rcodesign sign --p12-file ~/.p12/rustdesk-developer-id.p12 --p12-password-file ~/.p12/.cert-pass --code-signature-flags runtime ./rustdesk-{1}.dmg
    codesign -s "Developer ID Application: {0}" --force --options runtime ./rustdesk-{1}.dmg
    # https://appstoreconnect.apple.com/access/api
    # https://gregoryszorc.com/docs/apple-codesign/stable/apple_codesign_getting_started.html#apple-codesign-app-store-connect-api-key
    # p8 file is generated when you generate api key (can download only once)
    rcodesign notary-submit --api-key-path ../.p12/api-key.json  --staple rustdesk-{1}.dmg
    # verify:  spctl -a -t exec -v /Applications/RustDesk.app
    '''.format(pa, version))
                else:
                    print('Not signed')
            else:
                # build deb package
                system2(
                    'mv target/release/bundle/deb/rustdesk*.deb ./rustdesk.deb')
                system2('dpkg-deb -R rustdesk.deb tmpdeb')
                system2('mkdir -p tmpdeb/usr/share/rustdesk/files/systemd/')
                system2('mkdir -p tmpdeb/usr/share/icons/hicolor/256x256/apps/')
                system2('mkdir -p tmpdeb/usr/share/icons/hicolor/scalable/apps/')
                system2(
                    'cp res/rustdesk.service tmpdeb/usr/share/rustdesk/files/systemd/')
                system2(
                    'cp res/128x128@2x.png tmpdeb/usr/share/icons/hicolor/256x256/apps/rustdesk.png')
                system2(
                    'cp res/scalable.svg tmpdeb/usr/share/icons/hicolor/scalable/apps/rustdesk.svg')
                system2(
                    'cp res/rustdesk.desktop tmpdeb/usr/share/applications/rustdesk.desktop')
                system2(
                    'cp res/rustdesk-link.desktop tmpdeb/usr/share/applications/rustdesk-link.desktop')
                os.system('mkdir -p tmpdeb/etc/rustdesk/')
                os.system('cp -a res/startwm.sh tmpdeb/etc/rustdesk/')
                os.system('mkdir -p tmpdeb/etc/X11/rustdesk/')
                os.system('cp res/xorg.conf tmpdeb/etc/X11/rustdesk/')
                os.system('cp -a DEBIAN/* tmpdeb/DEBIAN/')
                os.system('mkdir -p tmpdeb/etc/pam.d/')
                os.system('cp pam.d/rustdesk.debian tmpdeb/etc/pam.d/rustdesk')
                system2('strip tmpdeb/usr/bin/rustdesk')
                system2('mkdir -p tmpdeb/usr/lib/rustdesk')
                system2('mv tmpdeb/usr/bin/rustdesk tmpdeb/usr/lib/rustdesk/')
                system2('cp libsciter-gtk.so tmpdeb/usr/lib/rustdesk/')
                md5_file('usr/share/rustdesk/files/systemd/rustdesk.service')
                md5_file('etc/rustdesk/startwm.sh')
                md5_file('etc/X11/rustdesk/xorg.conf')
                md5_file('etc/pam.d/rustdesk')
                md5_file('usr/lib/rustdesk/libsciter-gtk.so')
                system2('dpkg-deb -b tmpdeb rustdesk.deb; /bin/rm -rf tmpdeb/')
                os.rename('rustdesk.deb', 'rustdesk-%s.deb' % version)


def md5_file(fn):
    md5 = hashlib.md5(open('tmpdeb/' + fn, 'rb').read()).hexdigest()
    system2('echo "%s %s" >> tmpdeb/DEBIAN/md5sums' % (md5, fn))


if __name__ == "__main__":
    main()
