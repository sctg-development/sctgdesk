import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/desktop/widgets/tabbar_widget.dart';
import 'package:flutter_hbb/models/platform_model.dart';
import 'package:flutter_hbb/models/state_model.dart';
import 'package:get/get.dart';
import 'package:path/path.dart';
import 'package:url_launcher/url_launcher_string.dart';
import 'package:window_manager/window_manager.dart';

class InstallPage extends StatefulWidget {
  const InstallPage({Key? key}) : super(key: key);

  @override
  State<InstallPage> createState() => _InstallPageState();
}

class _InstallPageState extends State<InstallPage> {
  final tabController = DesktopTabController(tabType: DesktopTabType.main);

  @override
  void initState() {
    super.initState();
    Get.put<DesktopTabController>(tabController);
    const label = "install";
    tabController.add(TabInfo(
        key: label,
        label: label,
        closable: false,
        page: _InstallPageBody(
          key: const ValueKey(label),
        )));
  }

  @override
  void dispose() {
    super.dispose();
    Get.delete<DesktopTabController>();
  }

  @override
  Widget build(BuildContext context) {
    return DragToResizeArea(
      resizeEdgeSize: stateGlobal.resizeEdgeSize.value,
      child: Container(
        child: Scaffold(
            backgroundColor: Theme.of(context).colorScheme.background,
            body: DesktopTab(controller: tabController)),
      ),
    );
  }
}

class _InstallPageBody extends StatefulWidget {
  const _InstallPageBody({Key? key}) : super(key: key);

  @override
  State<_InstallPageBody> createState() => _InstallPageBodyState();
}

class _InstallPageBodyState extends State<_InstallPageBody>
    with WindowListener {
  late final TextEditingController controller;
  final RxBool startmenu = true.obs;
  final RxBool desktopicon = true.obs;
  final RxBool driverCert = true.obs;
  final RxBool showProgress = false.obs;
  final RxBool btnEnabled = true.obs;

  @override
  void initState() {
    windowManager.addListener(this);
    controller = TextEditingController(text: bind.installInstallPath());
    super.initState();
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    super.dispose();
  }

  @override
  void onWindowClose() {
    gFFI.close();
    super.onWindowClose();
    windowManager.setPreventClose(false);
    windowManager.close();
  }

  @override
  Widget build(BuildContext context) {
    final double em = 13;
    final btnFontSize = 0.9 * em;
    final double button_radius = 6;
    final isDarkTheme = MyTheme.currentThemeMode() == ThemeMode.dark;
    final buttonStyle = OutlinedButton.styleFrom(
        shape: RoundedRectangleBorder(
      borderRadius: BorderRadius.all(Radius.circular(button_radius)),
    ));
    final textColor = isDarkTheme ? null : Colors.black87;
    final dividerColor = isDarkTheme ? Colors.white70 : Colors.black87;
    return Scaffold(
        backgroundColor: null,
        body: SingleChildScrollView(
          child: Column(
            children: [
              Row(
                children: [
                  Text(
                    translate('Installation'),
                    style: TextStyle(
                        fontSize: 2 * em, fontWeight: FontWeight.w500),
                  ),
                ],
              ),
              Row(
                children: [
                  Text('${translate('Installation Path')}:')
                      .marginOnly(right: 10),
                  Expanded(
                    child: TextField(
                      controller: controller,
                      readOnly: true,
                      decoration: InputDecoration(
                        contentPadding: EdgeInsets.all(0.75 * em),
                      ),
                    ).marginOnly(right: 10),
                  ),
                  Obx(() => OutlinedButton(
                      onPressed: btnEnabled.value ? selectInstallPath : null,
                      style: buttonStyle,
                      child: Text(translate('Change Path'),
                          style: TextStyle(
                              color: textColor, fontSize: btnFontSize))))
                ],
              ).marginSymmetric(vertical: 2 * em),
              InkWell(
                borderRadius: BorderRadius.circular(6),
                onTap: () => startmenu.value = !startmenu.value,
                child: Row(
                  children: [
                    Obx(() => Checkbox(
                        visualDensity:
                            VisualDensity(horizontal: -4, vertical: -4),
                        value: startmenu.value,
                        onChanged: (b) {
                          if (b != null) startmenu.value = b;
                        }).marginOnly(right: 8)),
                    Expanded(
                        child: Text(translate('Create start menu shortcuts'))),
                  ],
                ),
              ).marginOnly(bottom: 7),
              InkWell(
                borderRadius: BorderRadius.circular(6),
                onTap: () => desktopicon.value = !desktopicon.value,
                child: Row(
                  children: [
                    Obx(() => Checkbox(
                        visualDensity:
                            VisualDensity(horizontal: -4, vertical: -4),
                        value: desktopicon.value,
                        onChanged: (b) {
                          if (b != null) desktopicon.value = b;
                        }).marginOnly(right: 8)),
                    Expanded(child: Text(translate('Create desktop icon'))),
                  ],
                ),
              ),
              Offstage(
                offstage: !Platform.isWindows,
                child: InkWell(
                  borderRadius: BorderRadius.circular(6),
                  onTap: () => driverCert.value = !driverCert.value,
                  child: Row(
                    children: [
                      Obx(() => Checkbox(
                          visualDensity:
                              VisualDensity(horizontal: -4, vertical: -4),
                          value: driverCert.value,
                          onChanged: (b) {
                            if (b != null) driverCert.value = b;
                          }).marginOnly(right: 8)),
                      Expanded(
                        child: Text(translate('idd_driver_tip')),
                      ),
                    ],
                  ),
                ).marginOnly(top: 7),
              ),
              InkWell(
                  hoverColor: Colors.transparent,
                  onTap: () => launchUrlString('https://rustdesk.com/privacy'),
                  child: Tooltip(
                    message: 'https://rustdesk.com/privacy',
                    child: Row(children: [
                      Icon(Icons.launch_outlined, size: 16)
                          .marginOnly(right: 5),
                      Text(
                        translate('End-user license agreement'),
                        style: const TextStyle(
                            decoration: TextDecoration.underline),
                      )
                    ]),
                  )).marginOnly(top: 2 * em),
              Row(children: [Text(translate('agreement_tip'))])
                  .marginOnly(top: em),
              Divider(color: dividerColor).marginSymmetric(vertical: 0.5 * em),
              Row(
                children: [
                  Expanded(
                      child: Obx(() => Offstage(
                            offstage: !showProgress.value,
                            child: LinearProgressIndicator(),
                          ))),
                  Obx(() => OutlinedButton(
                          onPressed: btnEnabled.value
                              ? () => windowManager.close()
                              : null,
                          style: buttonStyle,
                          child: Text(translate('Cancel'),
                              style: TextStyle(
                                  color: textColor, fontSize: btnFontSize)))
                      .marginSymmetric(horizontal: 2 * em)),
                  Obx(() => ElevatedButton(
                      onPressed: btnEnabled.value ? install : null,
                      style: ElevatedButton.styleFrom(
                          primary: MyTheme.button,
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.all(
                                Radius.circular(button_radius)),
                          )),
                      child: Text(
                        translate('Accept and Install'),
                        style: TextStyle(fontSize: btnFontSize),
                      ))),
                  Offstage(
                    offstage: bind.installShowRunWithoutInstall(),
                    child: Obx(() => OutlinedButton(
                            onPressed: btnEnabled.value
                                ? () => bind.installRunWithoutInstall()
                                : null,
                            style: buttonStyle,
                            child: Text(translate('Run without install'),
                                style: TextStyle(
                                    color: textColor, fontSize: btnFontSize)))
                        .marginOnly(left: 2 * em)),
                  ),
                ],
              )
            ],
          ).paddingSymmetric(horizontal: 8 * em, vertical: 2 * em),
        ));
  }

  void install() {
    do_install() {
      btnEnabled.value = false;
      showProgress.value = true;
      String args = '';
      if (startmenu.value) args += ' startmenu';
      if (desktopicon.value) args += ' desktopicon';
      if (driverCert.value) args += ' driverCert';
      bind.installInstallMe(options: args, path: controller.text);
    }

    if (driverCert.isTrue) {
      final tag = 'install-info-install-cert-confirm';
      final btns = [
        dialogButton(
          'Cancel',
          icon: Icon(Icons.close_rounded),
          onPressed: () => gFFI.dialogManager.dismissByTag(tag),
          isOutline: true,
        ),
        dialogButton(
          'OK',
          icon: Icon(Icons.done_rounded),
          onPressed: () {
            gFFI.dialogManager.dismissByTag(tag);
            do_install();
          },
          isOutline: false,
        ),
      ];
      gFFI.dialogManager.show(
        (setState, close, context) => CustomAlertDialog(
          title: null,
          content: SelectionArea(
              child:
                  msgboxContent('info', 'Warning', 'confirm_idd_driver_tip')),
          actions: btns,
          onCancel: close,
        ),
        tag: tag,
      );
    } else {
      do_install();
    }
  }

  void selectInstallPath() async {
    String? install_path = await FilePicker.platform
        .getDirectoryPath(initialDirectory: controller.text);
    if (install_path != null) {
      controller.text = join(install_path, await bind.mainGetAppName());
    }
  }
}
