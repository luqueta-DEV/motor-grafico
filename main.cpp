#include <iostream>
#include <GLFW/glfw3.h>
extern "C" {
   
    void* create_graphics_device();
    void render_graphics(void* device);
}

int main() {
    
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW!" << std::endl;
        return -1;
    }

   
    GLFWwindow* window = glfwCreateWindow(800, 600, "C++ + Rust Graphics", nullptr, nullptr);
    if (!window) {
        glfwTerminate();
        std::cerr << "Failed to create window!" << std::endl;
        return -1;
    }

    glfwMakeContextCurrent(window);

    
    void* graphics_device = create_graphics_device();

  
    while (!glfwWindowShouldClose(window)) {
        glClear(GL_COLOR_BUFFER_BIT);

        
        render_graphics(graphics_device);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwTerminate();
    return 0;
}

