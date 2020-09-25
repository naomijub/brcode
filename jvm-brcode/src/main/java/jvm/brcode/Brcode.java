package jvm.brcode;

import jnr.ffi.LibraryLoader;
import jnr.ffi.Runtime;

public class Brcode {
    private LibC libc;
    public Brcode() {
        libc = LibraryLoader.create(LibC.class).load("brcode");
    }

    public interface LibC {
        String json_from_brcode(String s);
        String json_to_brcode(String s);
    }

    public boolean someBrcodeMethod() {
        return true;
    }

    public String brcodeToJson(String brcode) {
        return libc.json_from_brcode(brcode);
    }

    public String jsonToBrcode(String json) {
        return libc.json_to_brcode(json);
    }
}
