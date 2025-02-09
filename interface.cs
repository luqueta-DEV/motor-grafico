using System;
using System.Runtime.InteropServices;

class GraphicsEngine
{
    
    [DllImport("your_rust_lib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr create_graphics_device();

    [DllImport("your_rust_lib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void render_graphics(IntPtr device);

    public static void Main()
    {
       
        IntPtr device = create_graphics_device();

       
        while (true)
        {
           
            render_graphics(device);
            
        }
    }
}
