//
// Copyright (c) ZeroC, Inc. All rights reserved.
//

#pragma once

module Demo
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
        RectType type;
    }

    interface Hello
    {
        void sayHello();
        void say(string text);
        RectProps calcRect(Rect rc);
    }
}
