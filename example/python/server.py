import os
import signal
import sys
import math
import Ice

filepath = os.path.dirname(os.path.abspath(__file__))
slice_dir = filepath + '/../slice/'
slice_files = [
    slice_dir + 'Demo.ice',
    slice_dir + 'Demo2.ice'
    
]

Ice.loadSlice('-I' + slice_dir + ' --all ' + ' '.join(slice_files))
import RustDemo

class AnotherDemoI(RustDemo.AnotherDemo):
    def baseException(self, current):
        raise RustDemo.DemoException("test 2")

class DemoI(RustDemo.Demo):
    def sayHello(self, current):
        print("Hello World!")
        
    def say(self, text, current):
        print(text)

    def calcRect(self, rc, current):
        props = RustDemo.RectProps()
        props.width = rc.right - rc.left
        props.height = rc.bottom - rc.top
        props.rectType = RustDemo.RectType.Square if props.width == props.height else RustDemo.RectType.Rect
        return props

    def add(self, a, b, current):
        return a + b
        
    def square(self, n, current):
        return n * n

    def squareRoot(self, n, current):
        return n > 0, math.sqrt(n)

    def sum(self, x, current):
        return sum(x)

    def getHello(self, x, current):
        if 'hello' in x:
            return x['hello']
        return math.nan

    def nativeException(self, current):
        raise NotImplementedError

    def baseException(self, current):
        raise RustDemo.DemoException("test")

    def derivedException(self, current):
        raise RustDemo.DerivedDemoException("test", "detail", False)

    def optionalSquare(self, n, current):
        return n * n if n != Ice.Unset else Ice.Unset

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
    adapter.add(AnotherDemoI(), Ice.stringToIdentity("demo2"))
    adapter.activate()
    communicator.waitForShutdown()