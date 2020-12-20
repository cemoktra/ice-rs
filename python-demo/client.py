import sys
import Ice

Ice.loadSlice('../Demo.ice')
import RustDemo

with Ice.initialize(sys.argv) as communicator:
    demo = RustDemo.DemoPrx.checkedCast(communicator.stringToProxy("demo:default -h localhost -p 10000"))
    x = demo.optionalSquare(Ice.Unset)
    y = demo.optionalSquare(2.0)