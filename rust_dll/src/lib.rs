use std::fs;

use jni::strings::JNIString;
// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JByteArray, JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jsize, jstring};

pub struct Test {
    x: Vec<u8>,
}

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_HelloWorld_hello<'local>(
    mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_vec<'local>(
    mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    class: JClass<'local>
) -> JObject<'local>{


    let x = Test{
        x: vec![1,2,3]
    };

    let class = { env.find_class("Test").unwrap() };
    let x2 = env.new_byte_array(x.x.len() as jsize).unwrap();
    
    let len = x.x.len();  // Get the length of the vector
    let ptr = x.x.as_ptr() as *const i8;  // Cast the pointer from *const u8 to *const i8
    let z = unsafe {
        std::slice::from_raw_parts(ptr, len)  // Convert the raw pointer into a slice
    };

    env.set_byte_array_region(&x2, 0, z).unwrap();

    env.new_object(class, "([B)V", &[(&x2).into()]).unwrap()

}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_test2<'local>(
    mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    class: JClass<'local>,
    input: JByteArray
){


    // Step 1: Get the length of the Java byte array (JByteArray)
    let array_length = env.get_array_length(&input).unwrap() as usize;
    
    // Step 2: Create a buffer (Vec<u8>) of the same length
    let mut buffer: Vec<i8> = vec![0; array_length];
    
    // Step 3: Copy the content of the Java byte array into the Rust buffer
    env.get_byte_array_region(&input, 0, &mut buffer).unwrap();

    let len = buffer.len();  // Get the length of the vector
    let ptr = buffer.as_ptr() as *const u8;  // Cast the pointer from *const u8 to *const i8
    let z = unsafe {
        std::slice::from_raw_parts(ptr, len)  // Convert the raw pointer into a slice
    }.to_vec();

    fs::write("test.jpg", z).unwrap();

}
