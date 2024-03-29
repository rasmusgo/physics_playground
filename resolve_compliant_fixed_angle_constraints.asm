xpbd_rigid_bodies::resolve_compliant_fixed_angle_constraints:
 push    rbp
 push    r15
 push    r14
 push    r13
 push    r12
 push    rsi
 push    rdi
 push    rbx
 sub     rsp, 1448
 lea     rbp, [rsp, +, 128]
 movaps  xmmword, ptr, [rbp, +, 1296], xmm15
 movaps  xmmword, ptr, [rbp, +, 1280], xmm14
 movaps  xmmword, ptr, [rbp, +, 1264], xmm13
 movaps  xmmword, ptr, [rbp, +, 1248], xmm12
 movaps  xmmword, ptr, [rbp, +, 1232], xmm11
 movaps  xmmword, ptr, [rbp, +, 1216], xmm10
 movaps  xmmword, ptr, [rbp, +, 1200], xmm9
 movaps  xmmword, ptr, [rbp, +, 1184], xmm8
 movaps  xmmword, ptr, [rbp, +, 1168], xmm7
 movaps  xmmword, ptr, [rbp, +, 1152], xmm6
 mov     qword, ptr, [rbp, +, 1144], -2
 mov     rbx, r9
 mov     rdi, r8
 mov     qword, ptr, [rbp, +, 1104], rdx
 mov     r12, rcx
 mov     rax, qword, ptr, [rbp, +, 1432]
 movaps  xmm0, xmmword, ptr, [rax]
 movaps  xmmword, ptr, [rbp, +, 768], xmm0
 mov     rax, qword, ptr, [rbp, +, 1424]
 movaps  xmm0, xmmword, ptr, [rax]
 movaps  xmmword, ptr, [rbp, +, 784], xmm0
 movss   xmm6, dword, ptr, [rbp, +, 1440]
 call    puffin::are_scopes_on
 test    al, al
 je      .LBB219_1
 lea     r15, [rip, +, __unnamed_115]
 mov     r14d, 60
 mov     edx, 60
 mov     rcx, r15
 call    core::str::<impl str>::rfind
 cmp     rax, 1
 jne     .LBB219_16
 mov     r9, rdx
 test    rdx, rdx
 je      .LBB219_8
 cmp     r9, 60
 jae     .LBB219_5
 lea     rax, [rip, +, __unnamed_115]
 cmp     byte, ptr, [r9, +, rax], -65
 jg      .LBB219_8
.LBB219_6:
 lea     rax, [rip, +, __unnamed_113]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_115]
 mov     edx, 60
 xor     r8d, r8d
 call    core::str::slice_error_fail
 ud2
.LBB219_1:
 xor     eax, eax
 mov     qword, ptr, [rbp, +, 1112], rax
 jmp     .LBB219_43
.LBB219_5:
 jne     .LBB219_6
.LBB219_8:
 lea     r15, [rip, +, __unnamed_115]
 mov     rcx, r15
 mov     rdx, r9
 call    core::str::<impl str>::rfind
 cmp     rax, 1
 jne     .LBB219_16
 mov     r8, rdx
 add     r8, 2
 je      .LBB219_15
 cmp     r8, 60
 jae     .LBB219_11
 lea     rax, [rip, +, __unnamed_115]
 cmp     byte, ptr, [rdx, +, rax, +, 2], -65
 jle     .LBB219_59
 mov     r14d, 58
 sub     r14, rdx
 jmp     .LBB219_15
.LBB219_11:
 jne     .LBB219_59
 xor     r14d, r14d
.LBB219_15:
 lea     r15, [rip, +, __unnamed_115]
 add     r15, r8
.LBB219_16:
 lea     rax, [rip, +, __unnamed_101+24]
 lea     r13, [rip, +, __unnamed_101]
.LBB219_17:
 movzx   ecx, byte, ptr, [rax, -, 1]
 test    cl, cl
 js      .LBB219_21
 dec     rax
 movzx   ecx, cl
 cmp     ecx, 92
 jne     .LBB219_25
 jmp     .LBB219_26
.LBB219_21:
 movzx   edx, byte, ptr, [rax, -, 2]
 cmp     dl, -64
 jge     .LBB219_22
 movzx   r8d, byte, ptr, [rax, -, 3]
 cmp     r8b, -64
 jge     .LBB219_31
 movzx   r9d, byte, ptr, [rax, -, 4]
 add     rax, -4
 and     r9d, 7
 shl     r9d, 6
 and     r8d, 63
 or      r8d, r9d
 jmp     .LBB219_32
.LBB219_22:
 add     rax, -2
 and     edx, 31
 jmp     .LBB219_23
.LBB219_31:
 add     rax, -3
 and     r8d, 15
.LBB219_32:
 shl     r8d, 6
 and     edx, 63
 or      edx, r8d
.LBB219_23:
 shl     edx, 6
 and     cl, 63
 movzx   ecx, cl
 or      ecx, edx
 cmp     ecx, 92
 je      .LBB219_26
.LBB219_25:
 cmp     ecx, 47
 je      .LBB219_26
 cmp     rax, r13
 jne     .LBB219_17
 mov     esi, 24
 call    puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY{{tls.shim}}
 cmp     qword, ptr, [rax], 0
 jne     .LBB219_38
.LBB219_39:
 mov     rcx, rax
 xor     edx, edx
 call    std::sys::common::thread_local::fast_local::Key<T>::try_initialize
 test    rax, rax
 jne     .LBB219_40
 lea     rax, [rip, +, __unnamed_26]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_27]
 lea     r9, [rip, +, __unnamed_28]
 lea     r8, [rbp, +, 1120]
 mov     edx, 70
 call    core::result::unwrap_failed
 ud2
.LBB219_26:
 lea     r13, [rip, +, __unnamed_101]
 sub     rax, r13
 mov     r8, rax
 mov     esi, 24
 inc     r8
 je      .LBB219_36
 cmp     r8, 24
 jae     .LBB219_28
 cmp     byte, ptr, [r8, +, r13], -65
 jle     .LBB219_60
 mov     esi, 23
 sub     rsi, rax
 jmp     .LBB219_36
.LBB219_28:
 jne     .LBB219_60
 xor     esi, esi
.LBB219_36:
 add     r13, r8
 call    puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY{{tls.shim}}
 cmp     qword, ptr, [rax], 0
 je      .LBB219_39
.LBB219_38:
 add     rax, 8
.LBB219_40:
 cmp     qword, ptr, [rax], 0
 jne     .LBB219_61
 mov     qword, ptr, [rax], -1
 mov     qword, ptr, [rbp, +, 1040], rax
 lea     rcx, [rax, +, 8]
 lea     rax, [rip, +, __unnamed_16]
 mov     qword, ptr, [rsp, +, 40], rax
 mov     qword, ptr, [rsp, +, 32], rsi
 mov     qword, ptr, [rsp, +, 48], 0
 mov     rdx, r15
 mov     r8, r14
 mov     r9, r13
 call    puffin::ThreadProfiler::begin_scope
 mov     qword, ptr, [rbp, +, 1016], rax
 mov     rax, qword, ptr, [rbp, +, 1040]
 inc     qword, ptr, [rax]
 mov     eax, 1
 mov     qword, ptr, [rbp, +, 1112], rax
.LBB219_43:
 mov     r14, qword, ptr, [rbp, +, 1104]
 shl     r14, 6
 add     r14, r12
 mov     qword, ptr, [rbp, +, 1104], r14
 mulss   xmm6, xmm6
 movss   dword, ptr, [rbp, +, 1100], xmm6
 movss   xmm11, dword, ptr, [rip, +, __real@3f800000]
 lea     r14, [rbp, +, 1120]
 cmp     r12, qword, ptr, [rbp, +, 1104]
 jne     .LBB219_49
 jmp     .LBB219_45
.LBB219_56:
 movaps  xmm8, xmm11
 movaps  xmm7, xmm11
 divss   xmm7, xmm0
 movaps  xmm0, xmm6
 call    acosf
 xorps   xmm1, xmm1
 sqrtss  xmm1, xmm7
 mulss   xmm1, xmm0
 movaps  xmm0, xmm6
 mulss   xmm0, xmm1
 shufps  xmm1, xmm1, 0
 movaps  xmm3, xmm12
 mulps   xmm3, xmm1
 mulss   xmm7, xmm12
 movaps  xmm2, xmm11
 subss   xmm2, xmm0
 mulss   xmm2, xmm7
 shufps  xmm2, xmm2, 0
 mulps   xmm2, xmm6
 addps   xmm2, xmm3
 mulps   xmm1, xmm6
 movaps  xmm3, xmm2
 shufps  xmm3, xmm2, 85
 movaps  xmm0, xmm1
 shufps  xmm0, xmm1, 85
 movhlps xmm1, xmm1
 movhlps xmm2, xmm2
.LBB219_57:
 xorps   xmm10, xmm10
 movss   xmm10, xmm3
 shufps  xmm10, xmm2, 76
 shufps  xmm10, xmm10, 120
 movaps  xmm4, xmm10
 xorps   xmm4, xmmword, ptr, [rip, +, __xmm@80000000800000008000000080000000]
 mulss   xmm3, xmm4
 xorps   xmm5, xmm5
 subss   xmm5, xmm3
 pshufd  xmm3, xmm4, 233
 mulps   xmm3, xmm2
 subss   xmm5, xmm3
 shufps  xmm3, xmm3, 85
 subss   xmm5, xmm3
 xorps   xmm8, xmm8
 sqrtss  xmm8, xmm5
 lea     r12, [r13, +, 64]
 ucomiss xmm8, xmm14
 jne     .LBB219_58
 jp      .LBB219_58
 cmp     r12, qword, ptr, [rbp, +, 1104]
 je      .LBB219_45
.LBB219_49:
 mov     r13, r12
 mov     rcx, qword, ptr, [r12, +, 16]
 cmp     rcx, rbx
 jae     .LBB219_52
 mov     r15, qword, ptr, [r13, +, 24]
 cmp     r15, rbx
 jae     .LBB219_51
 shl     rcx, 5
 lea     rax, [rdi, +, rcx]
 movaps  xmm8, xmmword, ptr, [rdi, +, rcx]
 mov     rsi, qword, ptr, [rax, +, 24]
 movss   xmm12, dword, ptr, [rax, +, 16]
 shl     r15, 5
 movaps  xmm0, xmmword, ptr, [rdi, +, r15]
 movaps  xmmword, ptr, [rbp, +, 976], xmm0
 movss   xmm0, dword, ptr, [rdi, +, r15, +, 16]
 movaps  xmmword, ptr, [rbp, +, 992], xmm0
 mov     r12, qword, ptr, [rdi, +, r15, +, 24]
 movss   xmm0, dword, ptr, [r13, +, 32]
 movss   xmm2, dword, ptr, [rip, +, __real@bf000000]
 mulss   xmm0, xmm2
 movsd   xmm1, qword, ptr, [r13, +, 36]
 movsd   xmm4, qword, ptr, [r13, +, 48]
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@0000000000000000bf000000bf000000]
 mulps   xmm1, xmm3
 movss   xmm5, dword, ptr, [r13, +, 44]
 mulss   xmm5, xmm2
 movaps  xmmword, ptr, [rbp, +, 1072], xmm5
 mulps   xmm4, xmm3
 movaps  xmmword, ptr, [rbp, +, 1056], xmm4
 movaps  xmm9, xmm11
 unpcklps xmm9, xmm0
 movlhps xmm9, xmm1
 mov     qword, ptr, [rbp, +, 1008], rax
 movss   xmm13, dword, ptr, [rax, +, 20]
 movss   xmm0, dword, ptr, [rdi, +, r15, +, 20]
 movaps  xmmword, ptr, [rbp, +, 1040], xmm0
 movaps  xmmword, ptr, [rbp, +, 544], xmm9
 mov     rcx, r14
 lea     rdx, [rbp, +, 544]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmm8
 shufps  xmm0, xmm8, 85
 movaps  xmm6, xmmword, ptr, [rbp, +, 1120]
 movaps  xmmword, ptr, [rbp, +, 688], xmm0
 mulps   xmm6, xmm0
 movaps  xmm7, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm6, xmm7
 movaps  xmmword, ptr, [rbp, +, 528], xmm9
 mov     rcx, r14
 lea     rdx, [rbp, +, 528]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm7, xmm8
 movaps  xmm0, xmm8
 shufps  xmm0, xmm8, 170
 movaps  xmm8, xmmword, ptr, [rbp, +, 1120]
 movaps  xmmword, ptr, [rbp, +, 736], xmm0
 mulps   xmm8, xmm0
 movaps  xmm10, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm8, xmm10
 addps   xmm8, xmm6
 movaps  xmmword, ptr, [rbp, +, 512], xmm9
 mov     rcx, r14
 lea     rdx, [rbp, +, 512]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm1, xmm7
 shufps  xmm1, xmm7, 255
 movaps  xmm0, xmmword, ptr, [rbp, +, 1120]
 movaps  xmmword, ptr, [rbp, +, 752], xmm1
 mulps   xmm0, xmm1
 movaps  xmm11, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm0, xmm11
 addps   xmm0, xmm8
 shufps  xmm13, xmm13, 0
 movaps  xmmword, ptr, [rbp, +, 592], xmm13
 movaps  xmm1, xmm13
 movaps  xmm14, xmmword, ptr, [rip, +, __xmm@00000000000000003f80000000000000]
 mulps   xmm1, xmm14
 addps   xmm1, xmm0
 movd    xmm0, esi
 pshufd  xmm0, xmm0, 0
 movdqa  xmmword, ptr, [rbp, +, 704], xmm0
 movaps  xmm15, xmmword, ptr, [rip, +, __xmm@000000003f8000000000000000000000]
 mulps   xmm0, xmm15
 addps   xmm0, xmm1
 shr     rsi, 32
 movd    xmm1, esi
 pshufd  xmm1, xmm1, 0
 movdqa  xmmword, ptr, [rbp, +, 720], xmm1
 movaps  xmm13, xmmword, ptr, [rip, +, __xmm@3f800000000000000000000000000000]
 mulps   xmm1, xmm13
 addps   xmm1, xmm0
 movaps  xmmword, ptr, [rbp, +, 576], xmm12
 movlhps xmm12, xmm7
 shufps  xmm12, xmm7, 8
 movaps  xmmword, ptr, [rbp, +, 1024], xmm9
 mulps   xmm12, xmm9
 addps   xmm12, xmm1
 movss   xmm6, dword, ptr, [rip, +, __real@3f800000]
 unpcklps xmm6, xmmword, ptr, [rbp, +, 1072]
 unpcklpd xmm6, xmmword, ptr, [rbp, +, 1056]
 movaps  xmmword, ptr, [rbp, +, 496], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 496]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmmword, ptr, [rbp, +, 976]
 movaps  xmm0, xmm11
 shufps  xmm0, xmm11, 85
 movaps  xmm8, xmmword, ptr, [rbp, +, 1120]
 movaps  xmmword, ptr, [rbp, +, 608], xmm0
 mulps   xmm8, xmm0
 mulps   xmm8, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 movaps  xmmword, ptr, [rbp, +, 480], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 480]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmm11
 shufps  xmm0, xmm11, 170
 movaps  xmm9, xmmword, ptr, [rbp, +, 1120]
 movaps  xmmword, ptr, [rbp, +, 656], xmm0
 mulps   xmm9, xmm0
 mulps   xmm9, xmm10
 addps   xmm9, xmm8
 movaps  xmmword, ptr, [rbp, +, 464], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 464]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm1, xmm11
 shufps  xmm1, xmm11, 255
 movaps  xmm0, xmmword, ptr, [rbp, +, 1120]
 movaps  xmmword, ptr, [rbp, +, 672], xmm1
 mulps   xmm0, xmm1
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 addps   xmm0, xmm9
 movaps  xmm1, xmmword, ptr, [rbp, +, 1040]
 shufps  xmm1, xmm1, 0
 movaps  xmmword, ptr, [rbp, +, 1040], xmm1
 mulps   xmm1, xmm14
 addps   xmm1, xmm0
 movd    xmm0, r12d
 pshufd  xmm0, xmm0, 0
 movdqa  xmmword, ptr, [rbp, +, 624], xmm0
 mulps   xmm0, xmm15
 addps   xmm0, xmm1
 shr     r12, 32
 movd    xmm1, r12d
 pshufd  xmm1, xmm1, 0
 movdqa  xmmword, ptr, [rbp, +, 640], xmm1
 mulps   xmm1, xmm13
 addps   xmm1, xmm0
 movaps  xmm10, xmmword, ptr, [rbp, +, 992]
 movlhps xmm10, xmm11
 shufps  xmm10, xmm11, 8
 mulps   xmm10, xmm6
 addps   xmm10, xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm7
 movaps  xmm8, xmm7
 movaps  xmm13, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 mulps   xmm8, xmm13
 mulps   xmm12, xmm13
 movaps  xmm15, xmm8
 shufps  xmm15, xmm8, 0
 movaps  xmm6, xmm15
 mulps   xmm6, xmm11
 movaps  xmmword, ptr, [rbp, +, 448], xmm11
 movaps  xmm13, xmm11
 mov     rcx, r14
 lea     rdx, [rbp, +, 448]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm9, xmm8
 shufps  xmm9, xmm8, 85
 movaps  xmm14, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm14, xmm9
 mulps   xmm14, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f800000bf800000]
 addps   xmm14, xmm6
 movaps  xmmword, ptr, [rbp, +, 432], xmm11
 mov     rcx, r14
 lea     rdx, [rbp, +, 432]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmm8
 shufps  xmm11, xmm8, 170
 movaps  xmm7, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm7, xmm11
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf800000bf800000]
 addps   xmm7, xmm14
 movaps  xmmword, ptr, [rbp, +, 416], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 416]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm8, xmm8, 255
 movaps  xmm6, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm6, xmm8
 mulps   xmm6, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f800000bf800000]
 addps   xmm6, xmm7
 mulps   xmm15, xmm10
 movaps  xmmword, ptr, [rbp, +, 400], xmm10
 mov     rcx, r14
 lea     rdx, [rbp, +, 400]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm9, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm9, xmmword, ptr, [rip, +, __xmm@bf8000003f800000bf8000003f800000]
 addps   xmm9, xmm15
 movaps  xmmword, ptr, [rbp, +, 384], xmm10
 mov     rcx, r14
 lea     rdx, [rbp, +, 384]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm11, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm11, xmmword, ptr, [rip, +, __xmm@3f800000bf800000bf8000003f800000]
 addps   xmm11, xmm9
 movss   xmm9, dword, ptr, [rip, +, __real@3f800000]
 movaps  xmmword, ptr, [rbp, +, 368], xmm10
 mov     rcx, r14
 lea     rdx, [rbp, +, 368]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm8, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm8, xmmword, ptr, [rip, +, __xmm@bf800000bf8000003f8000003f800000]
 addps   xmm8, xmm11
 movaps  xmm7, xmm12
 shufps  xmm7, xmm12, 0
 mulps   xmm7, xmm13
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 xorps   xmm14, xmm14
 addps   xmm7, xmm8
 movaps  xmmword, ptr, [rbp, +, 352], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 352]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm8, xmm12
 shufps  xmm8, xmm12, 85
 mulps   xmm8, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm8, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f8000003f800000]
 addps   xmm8, xmm7
 movaps  xmmword, ptr, [rbp, +, 336], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 336]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm7, xmm12
 shufps  xmm7, xmm12, 170
 mulps   xmm7, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf8000003f800000]
 addps   xmm7, xmm8
 movaps  xmmword, ptr, [rbp, +, 320], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 320]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm12, xmm12, 255
 mulps   xmm12, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm12, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f8000003f800000]
 addps   xmm12, xmm7
 movaps  xmm1, xmm6
 mulss   xmm1, xmm6
 movaps  xmm11, xmm9
 movaps  xmm0, xmm9
 subss   xmm0, xmm1
 ucomiss xmm14, xmm0
 jb      .LBB219_56
 movaps  xmm3, xmm12
 shufps  xmm3, xmm12, 85
 movaps  xmm2, xmm12
 unpckhpd xmm2, xmm12
 xorps   xmm1, xmm1
 xorps   xmm0, xmm0
 jmp     .LBB219_57
.LBB219_58:
 add     r13, 64
 add     r15, rdi
 xorps   xmm2, xmm2
 movss   xmm2, xmm0
 shufps  xmm2, xmm1, 76
 shufps  xmm2, xmm2, 120
 movaps  xmm0, xmm11
 divss   xmm0, xmm8
 shufps  xmm0, xmm0, 0
 mulps   xmm2, xmm0
 movaps  xmm14, xmm2
 mulps   xmm10, xmm0
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 mulps   xmm6, xmm0
 mulps   xmm12, xmm0
 movaps  xmmword, ptr, [rbp, +, 304], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 304]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm9, xmmword, ptr, [rbp, +, 1072]
 shufps  xmm9, xmm9, 0
 mulps   xmm9, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm9, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f8000003f800000]
 addps   xmm9, xmm12
 movaps  xmmword, ptr, [rbp, +, 288], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 288]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmmword, ptr, [rbp, +, 1056]
 movaps  xmm7, xmm11
 shufps  xmm7, xmm11, 0
 mulps   xmm7, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf8000003f800000]
 addps   xmm7, xmm9
 movaps  xmmword, ptr, [rbp, +, 272], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 272]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm11, xmm11, 85
 mulps   xmm11, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm11, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f8000003f800000]
 addps   xmm11, xmm7
 movaps  xmmword, ptr, [rbp, +, 1056], xmm11
 movaps  xmm13, xmmword, ptr, [rbp, +, 1024]
 movaps  xmmword, ptr, [rbp, +, 256], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 256]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm7, xmmword, ptr, [rbp, +, 1120]
 xorps   xmm11, xmm11
 mulps   xmm7, xmm11
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm7, xmm0
 movaps  xmmword, ptr, [rbp, +, 240], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 240]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm9, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm9, xmm11
 movaps  xmm15, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm9, xmm15
 addps   xmm9, xmm7
 movaps  xmmword, ptr, [rbp, +, 224], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 224]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm0, xmm11
 movaps  xmm1, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm0, xmm1
 movaps  xmm5, xmm1
 addps   xmm0, xmm9
 addps   xmm0, xmm11
 movaps  xmm1, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 mulps   xmm13, xmm1
 movaps  xmm3, xmm1
 addps   xmm13, xmm0
 movaps  xmm1, xmm10
 shufps  xmm1, xmm10, 96
 movaps  xmm0, xmm1
 movaps  xmm4, xmm1
 movaps  xmmword, ptr, [rbp, +, 928], xmm1
 mulps   xmm0, xmm11
 movaps  xmm1, xmmword, ptr, [rip, +, __xmm@bf8000003f80000000000000bf800000]
 mulps   xmm0, xmm1
 movaps  xmm2, xmm10
 shufps  xmm2, xmm10, 25
 movaps  xmm1, xmm2
 movaps  xmm7, xmm2
 movaps  xmmword, ptr, [rbp, +, 944], xmm2
 mulps   xmm1, xmm11
 movaps  xmm2, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf800000bf800000]
 mulps   xmm1, xmm2
 addps   xmm1, xmm0
 movaps  xmm2, xmm10
 shufps  xmm2, xmm10, 134
 movaps  xmm0, xmm2
 movaps  xmm9, xmm2
 movaps  xmmword, ptr, [rbp, +, 960], xmm2
 mulps   xmm0, xmm11
 movaps  xmm2, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f800000bf800000]
 mulps   xmm0, xmm2
 addps   xmm0, xmm1
 shufps  xmm10, xmm10, 144
 movaps  xmm12, xmm10
 mulps   xmm12, xmm3
 addps   xmm12, xmm0
 movaps  xmm0, xmm14
 shufps  xmm0, xmm14, 96
 movaps  xmmword, ptr, [rbp, +, 1024], xmm0
 mulps   xmm0, xmm11
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm0, xmm3
 movaps  xmm1, xmm14
 shufps  xmm1, xmm14, 25
 movaps  xmmword, ptr, [rbp, +, 880], xmm1
 mulps   xmm1, xmm11
 mulps   xmm1, xmm15
 addps   xmm1, xmm0
 movaps  xmm0, xmm14
 shufps  xmm0, xmm14, 134
 movaps  xmmword, ptr, [rbp, +, 896], xmm0
 mulps   xmm0, xmm11
 mulps   xmm0, xmm5
 addps   xmm0, xmm1
 movaps  xmm1, xmm13
 shufps  xmm1, xmm13, 0
 mulps   xmm1, xmm10
 movaps  xmm2, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf80000000000000]
 mulps   xmm1, xmm2
 addps   xmm1, xmm0
 movaps  xmm0, xmm13
 shufps  xmm0, xmm13, 85
 mulps   xmm0, xmm4
 mulps   xmm0, xmm3
 addps   xmm0, xmm1
 movaps  xmm1, xmm13
 shufps  xmm1, xmm13, 170
 mulps   xmm1, xmm7
 mulps   xmm1, xmm15
 addps   xmm1, xmm0
 movaps  xmm0, xmm13
 shufps  xmm0, xmm13, 255
 mulps   xmm0, xmm9
 mulps   xmm0, xmm5
 addps   xmm0, xmm1
 shufps  xmm14, xmm14, 144
 movaps  xmmword, ptr, [rbp, +, 912], xmm14
 mulps   xmm14, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 addps   xmm14, xmm0
 movaps  xmmword, ptr, [rbp, +, 1072], xmm14
 mulps   xmm13, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 movaps  xmm14, xmm12
 shufps  xmm14, xmm12, 0
 movaps  xmm7, xmm14
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@80000000800000008000000000000000]
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@8000000080000000800000003f800000]
 movaps  xmmword, ptr, [rbp, +, 208], xmm0
 movaps  xmm11, xmm0
 mov     rcx, r14
 lea     rdx, [rbp, +, 208]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm9, xmm12
 shufps  xmm9, xmm12, 85
 movaps  xmm15, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm15, xmm9
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f80000000000000]
 mulps   xmm15, xmm0
 addps   xmm15, xmm7
 movaps  xmmword, ptr, [rbp, +, 192], xmm11
 mov     rcx, r14
 lea     rdx, [rbp, +, 192]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmm12
 shufps  xmm11, xmm12, 170
 movaps  xmm7, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm7, xmm11
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf80000000000000]
 mulps   xmm7, xmm0
 addps   xmm7, xmm15
 movaps  xmm15, xmmword, ptr, [rip, +, __xmm@8000000080000000800000003f800000]
 movaps  xmmword, ptr, [rbp, +, 176], xmm15
 mov     rcx, r14
 lea     rdx, [rbp, +, 176]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm12, xmm12, 255
 movaps  xmm1, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm1, xmm12
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f80000000000000]
 mulps   xmm1, xmm0
 addps   xmm1, xmm7
 movaps  xmmword, ptr, [rbp, +, 864], xmm1
 mulps   xmm14, xmm13
 movaps  xmmword, ptr, [rbp, +, 160], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 160]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm9, xmmword, ptr, [rbp, +, 1120]
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf8000003f800000bf80000000000000]
 mulps   xmm9, xmm0
 addps   xmm9, xmm14
 movaps  xmmword, ptr, [rbp, +, 144], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 144]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm11, xmmword, ptr, [rbp, +, 1120]
 movaps  xmm14, xmmword, ptr, [rip, +, __xmm@3f800000bf800000bf80000000000000]
 mulps   xmm11, xmm14
 addps   xmm11, xmm9
 movaps  xmmword, ptr, [rbp, +, 128], xmm13
 mov     rcx, r14
 lea     rdx, [rbp, +, 128]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm12, xmmword, ptr, [rbp, +, 1120]
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf800000bf8000003f80000000000000]
 mulps   xmm12, xmm0
 addps   xmm12, xmm11
 movaps  xmm11, xmmword, ptr, [rbp, +, 1072]
 movaps  xmm7, xmm11
 mulss   xmm7, dword, ptr, [rip, +, __real@00000000]
 shufps  xmm7, xmm7, 0
 addps   xmm7, xmm12
 movaps  xmmword, ptr, [rbp, +, 112], xmm15
 mov     rcx, r14
 lea     rdx, [rbp, +, 112]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm9, xmm11
 shufps  xmm9, xmm11, 85
 mulps   xmm9, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm9, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f80000000000000]
 addps   xmm9, xmm7
 movaps  xmmword, ptr, [rbp, +, 96], xmm15
 mov     rcx, r14
 lea     rdx, [rbp, +, 96]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm7, xmm11
 shufps  xmm7, xmm11, 170
 mulps   xmm7, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf80000000000000]
 addps   xmm7, xmm9
 movaps  xmmword, ptr, [rbp, +, 80], xmm15
 mov     rcx, r14
 lea     rdx, [rbp, +, 80]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmm11
 shufps  xmm0, xmm11, 255
 mulps   xmm0, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f80000000000000]
 addps   xmm0, xmm7
 movaps  xmmword, ptr, [rbp, +, 1072], xmm0
 movaps  xmm0, xmm6
 shufps  xmm0, xmm6, 85
 movaps  xmm2, xmm0
 movaps  xmm5, xmmword, ptr, [rbp, +, 928]
 mulps   xmm2, xmm5
 mulps   xmm2, xmmword, ptr, [rip, +, __xmm@bf8000003f80000000000000bf800000]
 movaps  xmm1, xmm6
 shufps  xmm1, xmm6, 170
 movaps  xmm3, xmm1
 movaps  xmm9, xmmword, ptr, [rbp, +, 944]
 mulps   xmm3, xmm9
 mulps   xmm3, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf800000bf800000]
 addps   xmm3, xmm2
 movaps  xmm2, xmm6
 shufps  xmm2, xmm6, 255
 movaps  xmm4, xmm2
 movaps  xmm15, xmmword, ptr, [rbp, +, 960]
 mulps   xmm4, xmm15
 mulps   xmm4, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f800000bf800000]
 addps   xmm4, xmm3
 movaps  xmm12, xmm6
 shufps  xmm12, xmm6, 0
 movaps  xmm13, xmm12
 mulps   xmm13, xmm10
 movaps  xmm11, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 mulps   xmm13, xmm11
 addps   xmm13, xmm4
 mulps   xmm0, xmmword, ptr, [rbp, +, 1024]
 mulps   xmm1, xmmword, ptr, [rbp, +, 880]
 movaps  xmm4, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm0, xmm4
 movaps  xmm7, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm1, xmm7
 addps   xmm1, xmm0
 mulps   xmm2, xmmword, ptr, [rbp, +, 896]
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm2, xmm3
 addps   xmm2, xmm1
 movaps  xmm14, xmmword, ptr, [rbp, +, 1056]
 movaps  xmm0, xmm14
 shufps  xmm0, xmm14, 0
 mulps   xmm0, xmm10
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf80000000000000]
 addps   xmm0, xmm2
 movaps  xmm1, xmm14
 shufps  xmm1, xmm14, 85
 mulps   xmm1, xmm5
 mulps   xmm1, xmm4
 addps   xmm1, xmm0
 movaps  xmm0, xmm14
 shufps  xmm0, xmm14, 170
 mulps   xmm0, xmm9
 mulps   xmm0, xmm7
 addps   xmm0, xmm1
 movaps  xmm1, xmm14
 shufps  xmm1, xmm14, 255
 mulps   xmm1, xmm15
 mulps   xmm1, xmm3
 addps   xmm1, xmm0
 mulps   xmm12, xmmword, ptr, [rbp, +, 912]
 mulps   xmm12, xmm11
 addps   xmm12, xmm1
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 mulps   xmm6, xmm0
 mulps   xmm14, xmm0
 movaps  xmm15, xmm13
 shufps  xmm15, xmm13, 0
 movaps  xmm7, xmm6
 mulps   xmm7, xmm15
 movaps  xmmword, ptr, [rbp, +, 64], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 64]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm9, xmm13
 shufps  xmm9, xmm13, 85
 movaps  xmm10, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm10, xmm9
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f80000000000000]
 mulps   xmm10, xmm0
 addps   xmm10, xmm7
 movaps  xmmword, ptr, [rbp, +, 48], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 48]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmm13
 shufps  xmm11, xmm13, 170
 movaps  xmm7, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm7, xmm11
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf80000000000000]
 addps   xmm7, xmm10
 movaps  xmmword, ptr, [rbp, +, 32], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, +, 32]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm13, xmm13, 255
 movaps  xmm10, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm10, xmm13
 mulps   xmm10, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f80000000000000]
 addps   xmm10, xmm7
 mulps   xmm15, xmm14
 movaps  xmmword, ptr, [rbp, +, 16], xmm14
 mov     rcx, r14
 lea     rdx, [rbp, +, 16]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm9, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm9, xmmword, ptr, [rip, +, __xmm@bf8000003f800000bf80000000000000]
 addps   xmm9, xmm15
 movaps  xmmword, ptr, [rbp], xmm14
 mov     rcx, r14
 mov     rdx, rbp
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm11, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm11, xmmword, ptr, [rip, +, __xmm@3f800000bf800000bf80000000000000]
 addps   xmm11, xmm9
 movaps  xmmword, ptr, [rbp, -, 16], xmm14
 mov     rcx, r14
 lea     rdx, [rbp, -, 16]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm13, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm13, xmmword, ptr, [rip, +, __xmm@bf800000bf8000003f80000000000000]
 addps   xmm13, xmm11
 movaps  xmm0, xmm12
 shufps  xmm0, xmm12, 0
 mulps   xmm0, xmm6
 subps   xmm13, xmm0
 movaps  xmmword, ptr, [rbp, -, 32], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, -, 32]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm7, xmm12
 shufps  xmm7, xmm12, 85
 mulps   xmm7, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f80000000000000]
 addps   xmm7, xmm13
 movaps  xmmword, ptr, [rbp, -, 48], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, -, 48]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm9, xmm12
 shufps  xmm9, xmm12, 170
 mulps   xmm9, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm9, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf80000000000000]
 addps   xmm9, xmm7
 movaps  xmmword, ptr, [rbp, -, 64], xmm6
 mov     rcx, r14
 lea     rdx, [rbp, -, 64]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm12, xmm12, 255
 mulps   xmm12, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm12, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f80000000000000]
 addps   xmm12, xmm9
 movaps  xmm0, xmmword, ptr, [rbp, +, 848]
 movaps  xmm11, xmmword, ptr, [rbp, +, 1072]
 shufps  xmm0, xmm11, 51
 movaps  xmm1, xmm11
 shufps  xmm1, xmm0, 41
 movaps  xmmword, ptr, [rbp, +, 848], xmm1
 movaps  xmm0, xmmword, ptr, [rbp, +, 832]
 movaps  xmm3, xmmword, ptr, [rbp, +, 864]
 shufps  xmm0, xmm3, 51
 movaps  xmm2, xmm3
 movaps  xmm7, xmm3
 shufps  xmm2, xmm0, 41
 movaps  xmm5, xmm2
 movaps  xmmword, ptr, [rbp, +, 1056], xmm2
 movaps  xmm0, xmmword, ptr, [rbp, +, 816]
 shufps  xmm0, xmm12, 51
 movaps  xmm3, xmm12
 shufps  xmm3, xmm0, 41
 movaps  xmmword, ptr, [rbp, +, 816], xmm3
 movaps  xmm0, xmmword, ptr, [rbp, +, 800]
 shufps  xmm0, xmm10, 51
 movaps  xmm4, xmm10
 shufps  xmm4, xmm0, 41
 movaps  xmmword, ptr, [rbp, +, 1024], xmm4
 movaps  xmm6, xmmword, ptr, [rbp, +, 784]
 divps   xmm5, xmm6
 divps   xmm4, xmm6
 movaps  xmm6, xmm10
 movaps  xmm0, xmm7
 shufps  xmm6, xmm7, 17
 shufps  xmm6, xmm7, 226
 movaps  xmm7, xmm5
 unpcklps xmm7, xmm4
 mulps   xmm7, xmm6
 movaps  xmm6, xmm0
 unpckhps xmm6, xmm10
 movaps  xmm9, xmm4
 shufps  xmm9, xmm5, 17
 shufps  xmm9, xmm5, 226
 mulps   xmm9, xmm6
 addps   xmm9, xmm7
 shufps  xmm10, xmm0, 255
 shufps  xmm10, xmm0, 226
 movaps  xmm7, xmm5
 unpckhps xmm7, xmm4
 mulps   xmm7, xmm10
 addps   xmm7, xmm9
 movaps  xmm9, xmm1
 movaps  xmm10, xmmword, ptr, [rbp, +, 768]
 divps   xmm9, xmm10
 movaps  xmm6, xmm3
 divps   xmm6, xmm10
 movaps  xmm10, xmm12
 movaps  xmm0, xmm11
 shufps  xmm10, xmm11, 17
 shufps  xmm10, xmm11, 226
 movaps  xmm11, xmm9
 unpcklps xmm11, xmm6
 mulps   xmm11, xmm10
 addps   xmm11, xmm7
 movaps  xmm7, xmm0
 unpckhps xmm7, xmm12
 movaps  xmm10, xmm6
 shufps  xmm10, xmm9, 17
 shufps  xmm10, xmm9, 226
 mulps   xmm10, xmm7
 addps   xmm10, xmm11
 shufps  xmm12, xmm0, 255
 shufps  xmm12, xmm0, 226
 movaps  xmm7, xmm9
 unpckhps xmm7, xmm6
 mulps   xmm7, xmm12
 addps   xmm7, xmm10
 movaps  xmm10, xmm7
 shufps  xmm10, xmm7, 85
 addss   xmm10, xmm7
 movss   xmm7, dword, ptr, [r13, -, 4]
 divss   xmm7, dword, ptr, [rbp, +, 1100]
 addss   xmm7, xmm10
 divss   xmm8, xmm7
 shufps  xmm8, xmm8, 0
 movaps  xmm10, xmm8
 xorps   xmm10, xmmword, ptr, [rip, +, __xmm@80000000800000008000000080000000]
 mulps   xmm5, xmm10
 mulps   xmm10, xmm9
 mulps   xmm4, xmm8
 mulps   xmm8, xmm6
 movaps  xmm9, xmm10
 shufps  xmm9, xmm10, 96
 movaps  xmm0, xmmword, ptr, [rbp, +, 688]
 movaps  xmm6, xmm0
 mulps   xmm6, xmm9
 movaps  xmm1, xmmword, ptr, [rip, +, __xmm@bf8000003f80000000000000bf800000]
 mulps   xmm6, xmm1
 movaps  xmm12, xmm10
 shufps  xmm12, xmm10, 25
 movaps  xmm15, xmmword, ptr, [rbp, +, 736]
 movaps  xmm7, xmm15
 mulps   xmm7, xmm12
 movaps  xmm1, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf800000bf800000]
 mulps   xmm7, xmm1
 addps   xmm7, xmm6
 movaps  xmm11, xmm10
 shufps  xmm11, xmm10, 134
 movaps  xmm1, xmmword, ptr, [rbp, +, 752]
 movaps  xmm13, xmm1
 mulps   xmm13, xmm11
 movaps  xmm6, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f800000bf800000]
 mulps   xmm13, xmm6
 addps   xmm13, xmm7
 movaps  xmm14, xmmword, ptr, [rbp, +, 560]
 shufps  xmm14, xmm14, 0
 shufps  xmm10, xmm10, 144
 movaps  xmm6, xmm14
 mulps   xmm6, xmm10
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 mulps   xmm6, xmm3
 addps   xmm6, xmm13
 movaps  xmm7, xmm5
 shufps  xmm7, xmm5, 96
 mulps   xmm7, xmm0
 movaps  xmm13, xmm5
 shufps  xmm13, xmm5, 25
 mulps   xmm13, xmm15
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm7, xmm0
 movaps  xmm15, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm13, xmm15
 addps   xmm13, xmm7
 movaps  xmm7, xmm5
 shufps  xmm7, xmm5, 134
 mulps   xmm7, xmm1
 movaps  xmm2, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm7, xmm2
 addps   xmm7, xmm13
 movaps  xmm13, xmmword, ptr, [rbp, +, 576]
 shufps  xmm13, xmm13, 0
 mulps   xmm13, xmm10
 movaps  xmm1, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf80000000000000]
 mulps   xmm13, xmm1
 addps   xmm13, xmm7
 movaps  xmm7, xmmword, ptr, [rbp, +, 592]
 mulps   xmm7, xmm9
 mulps   xmm7, xmm0
 addps   xmm7, xmm13
 movaps  xmm0, xmmword, ptr, [rbp, +, 704]
 mulps   xmm0, xmm12
 mulps   xmm0, xmm15
 addps   xmm0, xmm7
 movaps  xmm7, xmmword, ptr, [rbp, +, 720]
 mulps   xmm7, xmm11
 mulps   xmm7, xmm2
 addps   xmm7, xmm0
 shufps  xmm5, xmm5, 144
 mulps   xmm5, xmm14
 mulps   xmm5, xmm3
 addps   xmm5, xmm7
 movaps  xmm12, xmmword, ptr, [rip, +, __xmm@3f0000003f0000003f0000003f000000]
 mulps   xmm6, xmm12
 mov     rax, qword, ptr, [rbp, +, 1008]
 movaps  xmm7, xmmword, ptr, [rax]
 subps   xmm7, xmm6
 mulps   xmm5, xmm12
 movaps  xmm6, xmmword, ptr, [rax, +, 16]
 subps   xmm6, xmm5
 movaps  xmmword, ptr, [rax], xmm7
 movaps  xmmword, ptr, [rax, +, 16], xmm6
 movaps  xmm6, xmm8
 shufps  xmm6, xmm8, 96
 movaps  xmm14, xmmword, ptr, [rbp, +, 608]
 movaps  xmm5, xmm14
 mulps   xmm5, xmm6
 mulps   xmm5, xmmword, ptr, [rip, +, __xmm@bf8000003f80000000000000bf800000]
 movaps  xmm9, xmm8
 shufps  xmm9, xmm8, 25
 movaps  xmm15, xmmword, ptr, [rbp, +, 656]
 movaps  xmm7, xmm15
 mulps   xmm7, xmm9
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf800000bf800000]
 addps   xmm7, xmm5
 movaps  xmm10, xmm8
 shufps  xmm10, xmm8, 134
 movaps  xmm0, xmmword, ptr, [rbp, +, 672]
 movaps  xmm11, xmm0
 mulps   xmm11, xmm10
 mulps   xmm11, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f800000bf800000]
 addps   xmm11, xmm7
 movaps  xmm13, xmmword, ptr, [rbp, +, 976]
 shufps  xmm13, xmm13, 0
 shufps  xmm8, xmm8, 144
 movaps  xmm5, xmm13
 mulps   xmm5, xmm8
 mulps   xmm5, xmm3
 movaps  xmm1, xmm3
 addps   xmm5, xmm11
 movaps  xmm7, xmm4
 shufps  xmm7, xmm4, 96
 mulps   xmm7, xmm14
 movaps  xmm11, xmm4
 shufps  xmm11, xmm4, 25
 mulps   xmm11, xmm15
 movaps  xmm2, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm7, xmm2
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm11, xmm3
 addps   xmm11, xmm7
 movaps  xmm7, xmm4
 shufps  xmm7, xmm4, 134
 mulps   xmm7, xmm0
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm7, xmm0
 addps   xmm7, xmm11
 movaps  xmm11, xmmword, ptr, [rbp, +, 992]
 shufps  xmm11, xmm11, 0
 mulps   xmm11, xmm8
 mulps   xmm11, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf80000000000000]
 addps   xmm11, xmm7
 movaps  xmm7, xmmword, ptr, [rbp, +, 1040]
 mulps   xmm7, xmm6
 mulps   xmm7, xmm2
 addps   xmm7, xmm11
 movaps  xmm6, xmmword, ptr, [rbp, +, 624]
 mulps   xmm6, xmm9
 movss   xmm9, dword, ptr, [rip, +, __real@3f800000]
 mulps   xmm6, xmm3
 addps   xmm6, xmm7
 movaps  xmm7, xmmword, ptr, [rbp, +, 640]
 mulps   xmm7, xmm10
 mulps   xmm7, xmm0
 addps   xmm7, xmm6
 shufps  xmm4, xmm4, 144
 mulps   xmm4, xmm13
 mulps   xmm4, xmm1
 addps   xmm4, xmm7
 mulps   xmm5, xmm12
 mulps   xmm4, xmm12
 movaps  xmm6, xmmword, ptr, [r15]
 subps   xmm6, xmm5
 movaps  xmm5, xmmword, ptr, [r15, +, 16]
 subps   xmm5, xmm4
 movaps  xmmword, ptr, [r15], xmm6
 movaps  xmmword, ptr, [r15, +, 16], xmm5
 movaps  xmm4, xmmword, ptr, [rax]
 movaps  xmm5, xmm4
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 mulps   xmm5, xmm0
 mulps   xmm5, xmm4
 movaps  xmm6, xmm5
 shufps  xmm6, xmm5, 255
 movaps  xmm7, xmm5
 unpckhpd xmm7, xmm5
 movaps  xmm8, xmm5
 shufps  xmm8, xmm5, 85
 subss   xmm5, xmm8
 subss   xmm5, xmm7
 subss   xmm5, xmm6
 sqrtss  xmm5, xmm5
 movaps  xmm6, xmm5
 mulss   xmm6, xmm5
 movaps  xmm7, xmm9
 divss   xmm7, xmm6
 mulss   xmm7, xmm5
 shufps  xmm7, xmm7, 0
 mulps   xmm4, xmm7
 mulps   xmm7, xmmword, ptr, [rax, +, 16]
 movaps  xmmword, ptr, [rax], xmm4
 movaps  xmmword, ptr, [rax, +, 16], xmm7
 movaps  xmm4, xmmword, ptr, [r15]
 movaps  xmm5, xmm4
 mulps   xmm5, xmm0
 mulps   xmm5, xmm4
 movaps  xmm6, xmm5
 movaps  xmm7, xmm5
 movaps  xmm8, xmm5
 shufps  xmm8, xmm5, 85
 subss   xmm5, xmm8
 shufps  xmm6, xmm6, 255
 movhlps xmm7, xmm7
 subss   xmm5, xmm7
 subss   xmm5, xmm6
 sqrtss  xmm5, xmm5
 movaps  xmm6, xmm5
 mulss   xmm6, xmm5
 movaps  xmm11, xmm9
 movaps  xmm7, xmm9
 divss   xmm7, xmm6
 mulss   xmm7, xmm5
 shufps  xmm7, xmm7, 0
 mulps   xmm4, xmm7
 mulps   xmm7, xmmword, ptr, [r15, +, 16]
 movaps  xmmword, ptr, [r15], xmm4
 movaps  xmmword, ptr, [r15, +, 16], xmm7
 movaps  xmm0, xmmword, ptr, [rbp, +, 1056]
 movaps  xmmword, ptr, [rbp, +, 832], xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 1024]
 movaps  xmmword, ptr, [rbp, +, 800], xmm0
 mov     r12, r13
 cmp     r12, qword, ptr, [rbp, +, 1104]
 jne     .LBB219_49
.LBB219_45:
 mov     rcx, qword, ptr, [rbp, +, 1112]
 mov     rdx, qword, ptr, [rbp, +, 1016]
 movaps  xmm6, xmmword, ptr, [rbp, +, 1152]
 movaps  xmm7, xmmword, ptr, [rbp, +, 1168]
 movaps  xmm8, xmmword, ptr, [rbp, +, 1184]
 movaps  xmm9, xmmword, ptr, [rbp, +, 1200]
 movaps  xmm10, xmmword, ptr, [rbp, +, 1216]
 movaps  xmm11, xmmword, ptr, [rbp, +, 1232]
 movaps  xmm12, xmmword, ptr, [rbp, +, 1248]
 movaps  xmm13, xmmword, ptr, [rbp, +, 1264]
 movaps  xmm14, xmmword, ptr, [rbp, +, 1280]
 movaps  xmm15, xmmword, ptr, [rbp, +, 1296]
 add     rsp, 1448
 pop     rbx
 pop     rdi
 pop     rsi
 pop     r12
 pop     r13
 pop     r14
 pop     r15
 pop     rbp
 jmp     _ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$puffin..ProfilerScope$GT$$GT$17hd4ceb1b4ba1a6a7eE
.LBB219_52:
 lea     r8, [rip, +, __unnamed_116]
 mov     rdx, rbx
 call    core::panicking::panic_bounds_check
 jmp     .LBB219_53
.LBB219_51:
 lea     r8, [rip, +, __unnamed_117]
 mov     rcx, r15
 mov     rdx, rbx
 call    core::panicking::panic_bounds_check
.LBB219_53:
 ud2
.LBB219_61:
 lea     rax, [rip, +, __unnamed_23]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_24]
 lea     r9, [rip, +, __unnamed_25]
 lea     r8, [rbp, +, 1120]
 mov     edx, 16
 call    core::result::unwrap_failed
 ud2
.LBB219_60:
 lea     rax, [rip, +, __unnamed_112]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_101]
 mov     edx, 24
 mov     r9d, 24
 call    core::str::slice_error_fail
 ud2
.LBB219_59:
 lea     rax, [rip, +, __unnamed_114]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_115]
 mov     edx, 60
 mov     r9d, 60
 call    core::str::slice_error_fail
 ud2
