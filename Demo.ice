#pragma once

module RustDemo
{
    enum RectType {
        Rect,
        Square
    }

    struct Rect {
        long left;
        long right;
        long top;
        long bottom;
    }

    struct RectProps {
        long width;
        long height;
        RectType rectType;
    }

    sequence<double> DoubleSeq;
    dictionary<string, double> TestDict;

    exception DemoException {
        string message;
    }
    exception DerivedDemoException extends DemoException {
        string detail;
        bool fatal;
    }

    interface Demo
    {
        // test simple call
        void sayHello();

        // test simple call with argument
        void say(string text);

        // test custom arguments
        RectProps calcRect(Rect rc);

        // test multiple args
        double add(double x, double y);
        
        // test output arguments
        void square(double x, out double y);
        bool squareRoot(double x, out double y);

        // test sequence
        double sum(DoubleSeq x);

        // test dict
        double getHello(TestDict x);

        // test exceptions
        void nativeException();
        void baseException() throws DemoException;
        void derivedException() throws DerivedDemoException;

        // test optional
        optional(2) double optionalSquare(optional(1) double n);
    }
}
