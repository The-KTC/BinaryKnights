import 'dart:ffi';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:my_app/bridge_generated.dart';

// testet für ios, macos oder windoof
const base = 'my_app';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
final dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
      ? DynamicLibrary.executable()
      : DynamicLibrary.open(path);
late final api = MyAppImpl(dylib);

void main() {
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'BinaryKnights´ Rust-Demo',
      home: Scaffold(
        appBar: AppBar(),
        body: Center(
          child: MyHomePage(),
        ),
      ),
    );
  }
}

class MyHomePage extends StatefulWidget {
  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String _displayText = "TEST";

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          TextButton(
            onPressed: () async {
              String getDataFromRust = await api.greetingFromRust();
              debugPrint(getDataFromRust);
              setState(() {
                _displayText = getDataFromRust;
              });
            },
            child: const Text('Hier drücken, um Rust auszuführen'),
          ),
          Text(
            _displayText,
            style: TextStyle(fontSize: 18),
          ),
        ],
      ),
    );
  }
}
