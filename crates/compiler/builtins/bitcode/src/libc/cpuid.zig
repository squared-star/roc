const builtin = @import("builtin");
const arch = builtin.cpu.arch;

// I couldn't manage to define this in a PIE friendly way with inline assembly.
// Instead, I am defining it as global assembly functions.
comptime {
    switch (arch) {
        .x86_64 => {
            asm (
                \\  # Check if AVX2 is supported.
                \\  # Returns 1 if AVX2 is supported, 0 otherwise.
                \\ .global supports_avx2;
                \\ supports_avx2:
                \\     # Save the EBX register.
                \\     push %rbx
                \\
                \\     # Call the CPUID instruction with the EAX register set to 7 and ECX set to 0.
                \\     # This will get the CPUID information for the current CPU.
                \\     mov $7, %eax
                \\     mov $0, %ecx
                \\     cpuid
                \\
                \\     # The AVX2 feature flag is located in the EBX register at bit 5.
                \\     bt $5, %ebx
                \\     jc .avx2_supported
                \\
                \\     # AVX2 is not supported.
                \\     pop %rbx
                \\     mov $0, %eax
                \\     ret
                \\
                \\     .avx2_supported:
                \\     pop %rbx
                \\     mov $1, %eax
                \\     ret
            );
            asm (
                \\  # Check if prefetchw is supported.
                \\  # Returns 1 if the prefetchw instruction is supported, 0 otherwise.
                \\ .global supports_prefetchw;
                \\ supports_prefetchw:
                \\     # Save the EBX register.
                \\     push %rbx
                \\
                \\     # Call the CPUID instruction with the EAX register set to 0x80000001 and ECX set to 0.
                \\     # This will get the CPUID information for the current CPU.
                \\     mov $0x80000001, %eax
                \\     mov $0, %ecx
                \\     cpuid
                \\
                \\     # The prefetchw feature flag is located in the ECX register at bit 8.
                \\     bt $8, %ecx
                \\     jc .prefetchw_supported
                \\
                \\     # AVX2 is not supported.
                \\     pop %rbx
                \\     mov $0, %eax
                \\     ret
                \\
                \\     .prefetchw_supported:
                \\     pop %rbx
                \\     mov $1, %eax
                \\     ret
            );
        },
        else => unreachable,
    }
}

pub extern fn supports_avx2() bool;
pub extern fn supports_prefetchw() bool;
