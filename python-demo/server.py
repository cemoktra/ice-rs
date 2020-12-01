#!/usr/bin/env python
#
# Copyright (c) ZeroC, Inc. All rights reserved.
#

import signal
import sys
import Ice

Ice.loadSlice('../Demo.ice')
import RustDemo


class DemoI(RustDemo.Demo):
    def sayHello(self, current):
        print("Hello World!")
        
    def say(self, text, current):
        print(text)

    def calcRect(self, rc, current):
        props = RustDemo.RectProps()
        props.width = rc.right - rc.left
        props.height = rc.bottom - rc.top
        props.type = RustDemo.RectType.Square if props.width == props.height else RustDemo.RectType.Rect
        return props


#
# Ice.initialize returns an initialized Ice communicator,
# the communicator is destroyed once it goes out of scope.
#
with Ice.initialize(sys.argv) as communicator:

    #
    # Install a signal handler to shutdown the communicator on Ctrl-C
    #
    signal.signal(signal.SIGINT, lambda signum, frame: communicator.shutdown())
    if hasattr(signal, 'SIGBREAK'):
        signal.signal(signal.SIGBREAK, lambda signum, frame: communicator.shutdown())
    adapter = communicator.createObjectAdapterWithEndpoints("Demo", "default -h localhost -p 10000")
    adapter.add(DemoI(), Ice.stringToIdentity("demo"))
    adapter.activate()
    communicator.waitForShutdown()
