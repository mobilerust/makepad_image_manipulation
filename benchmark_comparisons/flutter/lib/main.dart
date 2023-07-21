import 'dart:math';
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Image Grid'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  final List<String> imageList = const [
    'assets/images/image_1.png',
    'assets/images/image_2.png',
    'assets/images/image_3.png',
  ];

  final int numRows = 40;
  final int numColumns = 20;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      duration: const Duration(seconds: 5),
      vsync: this,
    )..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Widget buildImage(int index) {
    var child = Container(
      width: 20,
      height: 20,
      child: Image.asset(imageList[index % 3],
          fit: BoxFit.fill, filterQuality: FilterQuality.none),
    );

    return AnimatedBuilder(
      animation: _controller,
      child: child,
      builder: (BuildContext context, Widget? _widget) {
        if (index % 3 == 0) {
          return Transform.rotate(
            angle: _controller.value * 2.0 * pi,
            child: _widget!,
          );
        } else if (index % 3 == 1) {
          return Opacity(
            opacity: _controller.value,
            child: _widget!,
          );
        } else {
          return Transform.scale(
            scale: _controller.value,
            child: _widget!,
          );
        }
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: SingleChildScrollView(
        child: Column(
          children: List<Widget>.generate(numRows, (int rowIndex) {
            return Row(
              children: List<Widget>.generate(numColumns, (int columnIndex) {
                int index = rowIndex * numColumns + columnIndex;
                return buildImage(index);
              }),
            );
          }),
        ),
      ),
    );
  }
}
