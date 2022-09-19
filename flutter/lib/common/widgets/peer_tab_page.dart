import 'package:flutter/material.dart';
import 'package:flutter_hbb/common/widgets/peer_widget.dart';
import 'package:flutter_hbb/common/widgets/peercard_widget.dart';
import 'package:flutter_hbb/consts.dart';
import 'package:get/get.dart';

import '../../common.dart';
import '../../models/platform_model.dart';

class PeerTabPage extends StatefulWidget {
  final List<String> tabs;
  final List<Widget> children;
  const PeerTabPage({required this.tabs, required this.children, Key? key})
      : super(key: key);
  @override
  State<PeerTabPage> createState() => _PeerTabPageState();
}

class _PeerTabPageState extends State<PeerTabPage>
    with SingleTickerProviderStateMixin {
  late final PageController _pageController = PageController();
  final RxInt _tabIndex = 0.obs;

  @override
  void initState() {
    super.initState();
  }

  // hard code for now
  void _handleTabSelection(int index) {
    // reset search text
    peerSearchText.value = "";
    peerSearchTextController.clear();
    _tabIndex.value = index;
    _pageController.jumpToPage(index);
    switch (index) {
      case 0:
        bind.mainLoadRecentPeers();
        break;
      case 1:
        bind.mainLoadFavPeers();
        break;
      case 2:
        bind.mainDiscover();
        break;
      case 3:
        gFFI.abModel.getAb();
        break;
    }
  }

  @override
  void dispose() {
    _pageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      textBaseline: TextBaseline.ideographic,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(
          height: 28,
          child: Container(
              constraints: isDesktop ? null : kMobilePageConstraints,
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Expanded(child: _createTabBar(context)),
                  const SizedBox(width: 10),
                  const PeerSearchBar(),
                  Offstage(
                      offstage: !isDesktop,
                      child: _createPeerViewTypeSwitch(context)
                          .marginOnly(left: 13)),
                ],
              )),
        ),
        _createTabBarView(),
      ],
    );
  }

  Widget _createTabBar(BuildContext context) {
    return ListView(
        scrollDirection: Axis.horizontal,
        shrinkWrap: true,
        controller: ScrollController(),
        children: super.widget.tabs.asMap().entries.map((t) {
          return Obx(() => GestureDetector(
                child: Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8),
                    decoration: BoxDecoration(
                      color: _tabIndex.value == t.key
                          ? MyTheme.color(context).bg
                          : null,
                      borderRadius: BorderRadius.circular(2),
                    ),
                    child: Align(
                      alignment: Alignment.center,
                      child: Text(
                        t.value,
                        textAlign: TextAlign.center,
                        style: TextStyle(
                            height: 1,
                            fontSize: 14,
                            color: _tabIndex.value == t.key
                                ? MyTheme.color(context).text
                                : MyTheme.color(context).lightText),
                      ),
                    )),
                onTap: () => _handleTabSelection(t.key),
              ));
        }).toList());
  }

  Widget _createTabBarView() {
    final verticalMargin = isDesktop ? 12.0 : 6.0;
    return Expanded(
        child: PageView(
                physics: const BouncingScrollPhysics(),
                controller: _pageController,
                children: super.widget.children,
                onPageChanged: (to) => _tabIndex.value = to)
            .marginSymmetric(vertical: verticalMargin));
  }

  Widget _createPeerViewTypeSwitch(BuildContext context) {
    final activeDeco = BoxDecoration(color: MyTheme.color(context).bg);
    return Row(
      children: [
        Obx(
          () => Container(
            padding: const EdgeInsets.all(4.0),
            decoration:
                peerCardUiType.value == PeerUiType.grid ? activeDeco : null,
            child: InkWell(
                onTap: () {
                  peerCardUiType.value = PeerUiType.grid;
                },
                child: Icon(
                  Icons.grid_view_rounded,
                  size: 18,
                  color: peerCardUiType.value == PeerUiType.grid
                      ? MyTheme.color(context).text
                      : MyTheme.color(context).lightText,
                )),
          ),
        ),
        Obx(
          () => Container(
            padding: const EdgeInsets.all(4.0),
            decoration:
                peerCardUiType.value == PeerUiType.list ? activeDeco : null,
            child: InkWell(
                onTap: () {
                  peerCardUiType.value = PeerUiType.list;
                },
                child: Icon(
                  Icons.list,
                  size: 18,
                  color: peerCardUiType.value == PeerUiType.list
                      ? MyTheme.color(context).text
                      : MyTheme.color(context).lightText,
                )),
          ),
        ),
      ],
    );
  }
}

class PeerSearchBar extends StatefulWidget {
  const PeerSearchBar({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _PeerSearchBarState();
}

class _PeerSearchBarState extends State<PeerSearchBar> {
  var drawer = false;

  @override
  Widget build(BuildContext context) {
    return drawer
        ? _buildSearchBar()
        : IconButton(
            alignment: Alignment.centerRight,
            padding: const EdgeInsets.only(right: 2),
            onPressed: () {
              setState(() {
                drawer = true;
              });
            },
            icon: const Icon(
              Icons.search_rounded,
              color: MyTheme.dark,
            ));
  }

  Widget _buildSearchBar() {
    RxBool focused = false.obs;
    FocusNode focusNode = FocusNode();
    focusNode.addListener(() => focused.value = focusNode.hasFocus);
    return Container(
      width: 120,
      decoration: BoxDecoration(
        color: MyTheme.color(context).bg,
        borderRadius: BorderRadius.circular(6),
      ),
      child: Obx(() => Row(
            children: [
              Expanded(
                child: Row(
                  children: [
                    Icon(
                      Icons.search_rounded,
                      color: MyTheme.color(context).placeholder,
                    ).marginSymmetric(horizontal: 4),
                    Expanded(
                      child: TextField(
                        autofocus: true,
                        controller: peerSearchTextController,
                        onChanged: (searchText) {
                          peerSearchText.value = searchText;
                        },
                        focusNode: focusNode,
                        textAlign: TextAlign.start,
                        maxLines: 1,
                        cursorColor: MyTheme.color(context).lightText,
                        cursorHeight: 18,
                        cursorWidth: 1,
                        style: const TextStyle(fontSize: 14),
                        decoration: InputDecoration(
                          contentPadding:
                              const EdgeInsets.symmetric(vertical: 6),
                          hintText:
                              focused.value ? null : translate("Search ID"),
                          hintStyle: TextStyle(
                              fontSize: 14,
                              color: MyTheme.color(context).placeholder),
                          border: InputBorder.none,
                          isDense: true,
                        ),
                      ),
                    ),
                    // Icon(Icons.close),
                    IconButton(
                        alignment: Alignment.centerRight,
                        padding: const EdgeInsets.only(right: 2),
                        onPressed: () {
                          setState(() {
                            peerSearchTextController.clear();
                            peerSearchText.value = "";
                            drawer = false;
                          });
                        },
                        icon: const Icon(
                          Icons.close,
                          color: MyTheme.dark,
                        )),
                  ],
                ),
              )
            ],
          )),
    );
  }
}
