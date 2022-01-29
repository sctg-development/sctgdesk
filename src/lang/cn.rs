lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "状态"),
        ("Your Desktop", "你的桌面"),
        ("desk_tip", "你的桌面可以通过下面的ID和密码访问。"),
        ("Password", "密码"),
        ("Ready", "就绪"),
        ("connecting_status", "正在接入RustDesk网络..."),
        ("Enable Service", "允许服务"),
        ("Start Service", "启动服务"),
        ("Service is not running", "服务没有启动"),
        ("not_ready_status", "未就绪，请检查网络连接"),
        ("Control Remote Desktop", "控制远程桌面"),
        ("Transfer File", "传输文件"),
        ("Connect", "连接"),
        ("Recent Sessions", "最近访问过"),
        ("Address Book", "地址簿"),
        ("Confirmation", "确认"),
        ("TCP Tunneling", "TCP隧道"),
        ("Remove", "删除"),
        ("Refresh random password", "刷新随机密码"),
        ("Set your own password", "设置密码"),
        ("Enable Keyboard/Mouse", "允许控制键盘/鼠标"),
        ("Enable Clipboard", "允许同步剪贴板"),
        ("Enable File Transfer", "允许传输文件"),
        ("Enable TCP Tunneling", "允许建立TCP隧道"),
        ("IP Whitelisting", "IP白名单"),
        ("ID/Relay Server", "ID/中继服务器"),
        ("Stop service", "停止服务"),
        ("Change ID", "改变ID"),
        ("Website", "网站"),
        ("About", "关于"),
        ("Mute", "静音"),
        ("Audio Input", "音频输入"),
        ("ID Server", "ID服务器"),
        ("Relay Server", "中继服务器"),
        ("API Server", "API服务器"),
        ("invalid_http", "必须以http://或者https://开头"),
        ("Invalid IP", "无效IP"),
        ("id_change_tip", "只可以使用字母a-z, A-Z, 0-9, _ (下划线)。首字母必须是a-z, A-Z。长度在6与16之间。"),
        ("Invalid format", "无效格式"),
        ("This function is turned off by the server", "服务器关闭了此功能"),
        ("Not available", "已被占用"),
        ("Too frequent", "修改太频繁，请稍后再试"),
        ("Cancel", "取消"),
        ("Skip", "跳过"),
        ("Close", "关闭"),
        ("Retry", "再试"),
        ("OK", "确认"),
        ("Password Required", "需要密码"),
        ("Please enter your password", "请输入密码"),
        ("Remember password", "记住密码"),
        ("Wrong Password", "密码错误"),
        ("Do you want to enter again?", "还想输入一次吗?"),
        ("Connection Error", "连接错误"),
        ("Error", "错误"),
        ("Reset by the peer", "连接被对方关闭"),
        ("Connecting...", "正在连接..."),
        ("Connection in progress. Please wait.", "连接进行中，请稍等。"),
        ("Please try 1 minute later", "一分钟后再试"),
        ("Login Error", "登录错误"),
        ("Successful", "成功"),
        ("Connected, waiting for image...", "已连接，等待画面传输..."),
        ("Name", "文件名"),
        ("Modified", "修改时间"),
        ("Size", "大小"),
        ("Show Hidden Files", "显示隐藏文件"),
        ("Receive", "接受"),
        ("Send", "发送"),
        ("Remote Computer", "远程电脑"),
        ("Local Computer", "本地电脑"),
        ("Confirm Delete", "确认删除"),
        ("Are you sure you want to delete this file?", "是否删除此文件?"),
        ("Do this for all conflicts", "应用于其它冲突"),
        ("Deleting", "正在删除"),
        ("files", "文件"),
        ("Waiting", "等待..."),
        ("Finished", "完成"),
        ("Custom Image Quality", "设置画面质量"),
        ("Privacy mode", "隐私模式"),
        ("Block user input", "阻止用户输入"),
        ("Unblock user input", "取消阻止用户输入"),
        ("Adjust Window", "调节窗口"),
        ("Original", "原始比例"),
        ("Shrink", "收缩"),
        ("Stretch", "伸展"),
        ("Good image quality", "好画质"),
        ("Balanced", "一般画质"),
        ("Optimize reaction time", "优化反应时间"),
        ("Custom", "自定义画质"),
        ("Show remote cursor", "显示远程光标"),
        ("Disable clipboard", "禁止剪贴板"),
        ("Lock after session end", "断开后锁定远程电脑"),
        ("Insert", "插入"),
        ("Insert Lock", "锁定远程电脑"),
        ("Refresh", "刷新画面"),
        ("ID does not exist", "ID不存在"),
        ("Failed to connect to rendezvous server", "连接注册服务器失败"),
        ("Please try later", "请稍后再试"),
        ("Remote desktop is offline", "远程电脑不在线"),
        ("Key mismatch", "Key不匹配"),
        ("Timeout", "连接超时"),
        ("Failed to connect to relay server", "无法连接到中继服务器"),
        ("Failed to connect via rendezvous server", "无法通过注册服务器建立连接"),
        ("Failed to connect via relay server", "无法通过中继服务器建立连接"),
        ("Failed to make direct connection to remote desktop", "无法建立直接连接"),
        ("Set Password", "设置密码"),
        ("OS Password", "操作系统密码"),
        ("install_tip", "你正在运行未安装版本，由于UAC限制，作为被控端，会在某些情况下无法控制鼠标键盘，或者录制屏幕，请点击下面的按钮将RustDesk安装到系统，从而规避上述问题。"),
        ("Click to upgrade", "点击这里升级"),
        ("Click to download", "点击这里下载"),
        ("Click to update", "点击这里更新"),
        ("Configuration Permissions", "配置权限"),
        ("Configure", "配置"),
        ("config_acc", "为了能够远程控制你的桌面, 请给予RustDesk\"辅助功能\" 权限。"),
        ("config_screen", "为了能够远程访问你的桌面, 请给予RustDesk\"屏幕录制\" 权限。"),
        ("Installing ...", "安装 ..."),
        ("Install", "安装"),
        ("Installation", "安装"),
        ("Installation Path", "安装路径"),
        ("Create start menu shortcuts", "创建启动菜单快捷方式"),
        ("Create desktop icon", "创建桌面图标"),
        ("agreement_tip", "开始安装即表示接受许可协议。"),
        ("Accept and Install", "同意并安装"),
        ("End-user license agreement", "用户协议"),
        ("Generating ...", "正在产生 ..."),
        ("Your installation is lower version.", "你安装的版本比当前运行的低。"),
        ("not_close_tcp_tip", "请在使用隧道的时候，不要关闭本窗口"),
        ("Listening ...", "正在等待隧道连接 ..."),
        ("Remote Host", "远程主机"),
        ("Remote Port", "远程端口"),
        ("Action", "动作"),
        ("Add", "添加"),
        ("Local Port", "本地端口"),
        ("setup_server_tip", "如果需要更快连接速度，你可以选择自建服务器"),
        ("Too short, at least 6 characters.", "太短了，至少6个字符"),
        ("The confirmation is not identical.", "两次输入不匹配"),
        ("Permissions", "权限"),
        ("Accept", "接受"),
        ("Dismiss", "拒绝"),
        ("Disconnect", "断开连接"),
        ("Allow using keyboard and mouse", "允许使用键盘鼠标"),
        ("Allow using clipboard", "允许使用剪贴板"),
        ("Allow hearing sound", "允许听到声音"),
        ("Connected", "已经连接"),
        ("Direct and encrypted connection", "加密直连"),
        ("Relayed and encrypted connection", "加密中继连接"),
        ("Direct and unencrypted connection", "非加密直连"),
        ("Relayed and unencrypted connection", "非加密中继连接"),
        ("Enter Remote ID", "输入对方ID"),
        ("Enter your password", "输入密码"),
        ("Logging in...", "正在登录..."),
        ("Enable RDP session sharing", "允许RDP会话共享"),
        ("Auto Login", "自动登录（设置断开后锁定才有效）"),
        ("Enable Direct IP Access", "允许IP直接访问"),
        ("Rename", "改名"),
        ("Space", "空格"),
        ("Create Desktop Shortcut", "创建桌面快捷方式"),
        ("Change Path", "改变路径"),
        ("Create Folder", "创建文件夹"),
        ("Please enter the folder name", "请输入文件夹名称"),
        ("Fix it", "修复"),
        ("Warning", "警告"),
        ("Login screen using Wayland is not supported", "不支持使用 Wayland 登录界面"),
        ("Reboot required", "重启后才能生效"),
        ("Unsupported display server ", "不支持当前显示服务器"),
        ("x11 expected", "请切换到 x11"),
        ("Port", "端口"),
        ("Settings", "设置"),
        ("Username", " 用户名"),
        ("Invalid port", "无效端口"),
        ("Closed manually by the peer", "被对方手动关闭"),
        ("Enable remote configuration modification", "允许远程修改配置"),
        ("Run without install", "无安装运行"),
        ("Always connected via relay", "强制走中继连接"),
        ("Always connect via relay", "强制走中继连接"),
        ("whitelist_tip", "只有白名单里的ip才能访问我"),
        ("Login", "登录"),
        ("Logout", "登出"),
        ("Tags", "标签"),
        ("Search ID", "查找ID"),
        ("Current Wayland display server is not supported", "不支持 Wayland 显示服务器"),
        ("whitelist_sep", "可以使用逗号，分号，空格或者换行符作为分隔符"),
        ("Add ID", "增加ID"),
        ("Add Tag", "增加标签"),
        ("Unselect all tags", "取消选择所有标签"),
        ("Network error", "网络错误"),
        ("Username missed", "用户名没有填写"),
        ("Password missed", "密码没有填写"),
        ("Wrong credentials", "用户名或者密码错误"),
        ("Edit Tag", "修改标签"),
        ("Unremember Password", "忘掉密码"),
        ("Favorites", "收藏"),
        ("Add to Favorites", "加入到收藏"),
        ("Remove from Favorites", "从收藏中删除"),
        ("Empty", "空空如也"),
        ("Invalid folder name", "无效文件夹名称"),
        ("Socks5 Proxy", "Socks5 代理"),
        ("Hostname", "主机名"),
        ("Discovered", "已发现"),
        ("install_daemon_tip", "为了开机启动，请安装系统服务。"),
    ].iter().cloned().collect();
    }
