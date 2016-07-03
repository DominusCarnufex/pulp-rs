#include <stdio.h>
#include <stdint.h>

struct HeapVar  {
    const unsigned char *pointer;
    size_t size;
};

struct Option   {
    unsigned char valid;
    int64_t value;
};

struct PulpResult   {
     unsigned char which;
     struct HeapVar value;
};

extern struct PulpResult run_v0_1_0(struct HeapVar);

int main()  {
    unsigned char bytecode[0x4e] = {
        0x50, 0x55, 0x4c, 0x50,
        0x00, 0x01, 0x00,
        0x00,
        0x57, 0x6e, 0xbc, 0xfa,
        0x32, 0x00, 0x7b, 0xf9, 0x73, 0xca, 0x8b, 0x5f,
        0x09, 0xe0, 0x54, 0x09, 0x3a, 0xab, 0xf2, 0x60,
        0x32, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x04, 0x6d, 0x61, 0x69, 0x6e,
        0x04, 0x00, 0x00, 0x00,
        0x16, 0x00, 0x02, 0x00,
          0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,    
        0x0b, 0x00, 0x03, 0x00,
          0x0a, 0x00, 0x00,
          0x0a, 0x01, 0x00,
          0x30
    };

    struct HeapVar code = {
        (const unsigned char*) bytecode,
        0x4e
    };

    struct PulpResult res = run_v0_1_0(code);

    if (res.which == 0) {
        struct Option opt = *((struct Option *) res.value.pointer);
        if (!opt.valid || opt.value != 3)   {
            printf("Test échoué : résultat incorrect.\n");
            return 1;
        } else {
            return 0;
        }
    } else if (res.which == 1)  {
        uint32_t err = *(res.value.pointer);
        printf("Erreur du programme : %d\n", err);
        return 1;
    } else  {
        printf("Erreur : %s\n", res.value.pointer);
        return 1;
    }
}
