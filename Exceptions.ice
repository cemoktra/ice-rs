#pragma once

module RustDemo
{
    exception DemoException {
        string message;
    }
    exception DerivedDemoException extends DemoException {
        string detail;
        bool fatal;
    }
}
