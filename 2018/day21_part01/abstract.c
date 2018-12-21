void foo(void) {
    int r0=0, r1=0, r2=0, r3=0, r5=0;

    r2 = 0;
    do { // ASM ROW 6
        r5 = r2 | 65536;
        r2 = 5234604;

        // ASM ROW 8
        r3 = r5 & 255;  // get the last byte from r5
        r2 &= r3;       // mask r2 according to the last byte from r5
        r2 &= 16777215; // mask the last 4 bytes from r2
        r2 *= 65899;    // multiply by a big prime
        r2 &= 16777215; // mask the last 4 bytes from r2

        if (r5 >= 256) { // ASM ROW 13
            for (r3 = 0; r1 > r5; r3++) { // ASM ROW 18; ROW 20; ROW 24
                r1 = r3 + 1; // ASM ROW 18
                r1 <<= 8;
            }
            r5 = r3;
            // GOTO 8
        }
    } while (r0 != r2); // ASM ROW 28
}