import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

class HelloWorld {
    // This declares that the static `hello` method will be provided
    // a native library.
    private static native String hello(String input);
    private static native Test vec();
    private static native void test2(byte[] input);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("rust_dll_demo");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) throws IOException {
        File file = new File("C:\\Users\\nickw\\Downloads\\unnamed(1).jpg");
        byte[] bytes = Files.readAllBytes(file.toPath());
        test2(bytes);
        String output = HelloWorld.hello("josh");
        Test t = HelloWorld.vec();
        System.out.println(t.x[2]);
        System.out.println(output);
    }
}