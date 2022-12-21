import 'dart:convert';
import 'package:flutter/services.dart';

import '../bridge/bridge.dart';

class PatternCollectList {
  late List _nameList;
  final List<Pattern> _patternList = [];

  Future<void> init() async {
    final index = await rootBundle.loadString('assets/pattern/index.json');
    _nameList = jsonDecode(index);
  }

  int length() => _nameList.length;

  Future<Pattern> getPattern(int index) async {
    if (index < _patternList.length - 1) {
      return _patternList[index];
    } else {
      final pattern = await rootBundle.loadStructuredData(
        'assets/pattern/${_nameList[index]}',
        (value) => bridge.decodeRle(rle: value),
      );

      _patternList.add(pattern);

      return pattern;
    }
  }
}
