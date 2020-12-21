#pragma once

#include <Exceptions.ice>

module RustDemo
{
    interface AnotherDemo
    {
        void baseException() throws DemoException;
    }
}
