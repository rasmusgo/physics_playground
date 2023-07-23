xpbd_rigid_bodies::resolve_compliant_fixed_angle_constraints:
 push    rbp
 push    r15
 push    r14
 push    r13
 push    r12
 push    rsi
 push    rdi
 push    rbx
 sub     rsp, 1576
 lea     rbp, [rsp, +, 128]
 movaps  xmmword, ptr, [rbp, +, 1424], xmm15
 movaps  xmmword, ptr, [rbp, +, 1408], xmm14
 movaps  xmmword, ptr, [rbp, +, 1392], xmm13
 movaps  xmmword, ptr, [rbp, +, 1376], xmm12
 movaps  xmmword, ptr, [rbp, +, 1360], xmm11
 movaps  xmmword, ptr, [rbp, +, 1344], xmm10
 movaps  xmmword, ptr, [rbp, +, 1328], xmm9
 movaps  xmmword, ptr, [rbp, +, 1312], xmm8
 movaps  xmmword, ptr, [rbp, +, 1296], xmm7
 movaps  xmmword, ptr, [rbp, +, 1280], xmm6
 mov     qword, ptr, [rbp, +, 1272], -2
 mov     rbx, r9
 mov     rdi, r8
 mov     qword, ptr, [rbp, +, 1208], rdx
 mov     r14, rcx
 mov     rax, qword, ptr, [rbp, +, 1560]
 movaps  xmm0, xmmword, ptr, [rax]
 movaps  xmmword, ptr, [rbp, +, 1008], xmm0
 mov     rax, qword, ptr, [rbp, +, 1552]
 movaps  xmm0, xmmword, ptr, [rax]
 movaps  xmmword, ptr, [rbp, +, 1024], xmm0
 movss   xmm0, dword, ptr, [rbp, +, 1568]
 movss   dword, ptr, [rbp, +, 1228], xmm0
 call    puffin::are_scopes_on
 test    al, al
 je      .LBB219_1
 lea     r12, [rip, +, __unnamed_115]
 mov     r15d, 60
 mov     edx, 60
 mov     rcx, r12
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
 mov     qword, ptr, [rbp, +, 1216], rax
 jmp     .LBB219_43
.LBB219_5:
 jne     .LBB219_6
.LBB219_8:
 lea     r12, [rip, +, __unnamed_115]
 mov     rcx, r12
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
 jle     .LBB219_62
 mov     r15d, 58
 sub     r15, rdx
 jmp     .LBB219_15
.LBB219_11:
 jne     .LBB219_62
 xor     r15d, r15d
.LBB219_15:
 lea     r12, [rip, +, __unnamed_115]
 add     r12, r8
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
 lea     r8, [rbp, +, 1232]
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
 jle     .LBB219_63
 mov     esi, 23
 sub     rsi, rax
 jmp     .LBB219_36
.LBB219_28:
 jne     .LBB219_63
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
 jne     .LBB219_64
 mov     qword, ptr, [rax], -1
 mov     qword, ptr, [rbp, +, 1152], rax
 lea     rcx, [rax, +, 8]
 lea     rax, [rip, +, __unnamed_16]
 mov     qword, ptr, [rsp, +, 40], rax
 mov     qword, ptr, [rsp, +, 32], rsi
 mov     qword, ptr, [rsp, +, 48], 0
 mov     rdx, r12
 mov     r8, r15
 mov     r9, r13
 call    puffin::ThreadProfiler::begin_scope
 mov     qword, ptr, [rbp, +, 1144], rax
 mov     rax, qword, ptr, [rbp, +, 1152]
 inc     qword, ptr, [rax]
 mov     eax, 1
 mov     qword, ptr, [rbp, +, 1216], rax
.LBB219_43:
 mov     r15, qword, ptr, [rbp, +, 1208]
 shl     r15, 6
 add     r15, r14
 mov     qword, ptr, [rbp, +, 1208], r15
 movss   xmm0, dword, ptr, [rbp, +, 1228]
 mulss   xmm0, xmm0
 movss   dword, ptr, [rbp, +, 1228], xmm0
 lea     r15, [rbp, +, 1232]
 mov     qword, ptr, [rbp, +, 1136], rbx
 jmp     .LBB219_44
.LBB219_57:
 mov     r14, rbx
 mov     rbx, qword, ptr, [rbp, +, 1136]
.LBB219_44:
 cmp     r14, qword, ptr, [rbp, +, 1208]
 je      .LBB219_45
 mov     rcx, qword, ptr, [r14, +, 16]
 cmp     rcx, rbx
 jae     .LBB219_52
 mov     r12, qword, ptr, [r14, +, 24]
 cmp     r12, rbx
 jae     .LBB219_51
 shl     rcx, 5
 lea     r13, [rdi, +, rcx]
 movaps  xmm7, xmmword, ptr, [rdi, +, rcx]
 mov     rsi, qword, ptr, [r13, +, 24]
 shl     r12, 5
 movaps  xmm8, xmmword, ptr, [rdi, +, r12]
 mov     rbx, qword, ptr, [rdi, +, r12, +, 24]
 movss   xmm0, dword, ptr, [r14, +, 32]
 movss   xmm2, dword, ptr, [rip, +, __real@bf000000]
 mulss   xmm0, xmm2
 movsd   xmm1, qword, ptr, [r14, +, 36]
 movsd   xmm4, qword, ptr, [r14, +, 48]
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@0000000000000000bf000000bf000000]
 mulps   xmm1, xmm3
 movss   xmm5, dword, ptr, [r14, +, 44]
 mulss   xmm5, xmm2
 movaps  xmmword, ptr, [rbp, +, 1184], xmm5
 mulps   xmm4, xmm3
 movaps  xmmword, ptr, [rbp, +, 1168], xmm4
 movss   xmm10, dword, ptr, [rip, +, __real@3f800000]
 unpcklps xmm10, xmm0
 movlhps xmm10, xmm1
 movss   xmm0, dword, ptr, [r13, +, 16]
 movaps  xmmword, ptr, [rbp, +, 1056], xmm0
 movss   xmm9, dword, ptr, [r13, +, 20]
 movss   xmm0, dword, ptr, [rdi, +, r12, +, 20]
 movaps  xmmword, ptr, [rbp, +, 1152], xmm0
 movss   xmm0, dword, ptr, [rdi, +, r12, +, 16]
 movaps  xmmword, ptr, [rbp, +, 1040], xmm0
 movaps  xmmword, ptr, [rbp, +, 640], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 640]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmm7
 shufps  xmm0, xmm7, 85
 movaps  xmm6, xmmword, ptr, [rbp, +, 1232]
 movaps  xmmword, ptr, [rbp, +, 784], xmm0
 mulps   xmm6, xmm0
 movaps  xmm11, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm6, xmm11
 movaps  xmmword, ptr, [rbp, +, 624], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 624]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmm7
 movaps  xmm0, xmm7
 shufps  xmm0, xmm7, 170
 movaps  xmm7, xmmword, ptr, [rbp, +, 1232]
 movaps  xmmword, ptr, [rbp, +, 832], xmm0
 mulps   xmm7, xmm0
 movaps  xmm12, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm7, xmm12
 addps   xmm7, xmm6
 movaps  xmmword, ptr, [rbp, +, 608], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 608]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm1, xmm11
 shufps  xmm1, xmm11, 255
 movaps  xmm0, xmmword, ptr, [rbp, +, 1232]
 movaps  xmmword, ptr, [rbp, +, 848], xmm1
 mulps   xmm0, xmm1
 movaps  xmm13, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm0, xmm13
 addps   xmm0, xmm7
 shufps  xmm9, xmm9, 0
 movaps  xmmword, ptr, [rbp, +, 688], xmm9
 movaps  xmm1, xmm9
 movaps  xmm14, xmmword, ptr, [rip, +, __xmm@00000000000000003f80000000000000]
 mulps   xmm1, xmm14
 addps   xmm1, xmm0
 movd    xmm0, esi
 pshufd  xmm0, xmm0, 0
 movdqa  xmmword, ptr, [rbp, +, 800], xmm0
 movaps  xmm15, xmmword, ptr, [rip, +, __xmm@000000003f8000000000000000000000]
 mulps   xmm0, xmm15
 addps   xmm0, xmm1
 shr     rsi, 32
 movd    xmm1, esi
 pshufd  xmm1, xmm1, 0
 movdqa  xmmword, ptr, [rbp, +, 816], xmm1
 movaps  xmm2, xmmword, ptr, [rip, +, __xmm@3f800000000000000000000000000000]
 mulps   xmm1, xmm2
 addps   xmm1, xmm0
 movaps  xmm9, xmmword, ptr, [rbp, +, 1056]
 movlhps xmm9, xmm11
 shufps  xmm9, xmm11, 8
 movaps  xmmword, ptr, [rbp, +, 1120], xmm10
 mulps   xmm9, xmm10
 addps   xmm9, xmm1
 movss   xmm6, dword, ptr, [rip, +, __real@3f800000]
 unpcklps xmm6, xmmword, ptr, [rbp, +, 1184]
 unpcklpd xmm6, xmmword, ptr, [rbp, +, 1168]
 movaps  xmmword, ptr, [rbp, +, 592], xmm6
 mov     rcx, r15
 lea     rdx, [rbp, +, 592]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmm8
 shufps  xmm0, xmm8, 85
 movaps  xmm7, xmmword, ptr, [rbp, +, 1232]
 movaps  xmmword, ptr, [rbp, +, 704], xmm0
 mulps   xmm7, xmm0
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 movaps  xmmword, ptr, [rbp, +, 576], xmm6
 mov     rcx, r15
 lea     rdx, [rbp, +, 576]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmm8
 shufps  xmm0, xmm8, 170
 movaps  xmm10, xmmword, ptr, [rbp, +, 1232]
 movaps  xmmword, ptr, [rbp, +, 752], xmm0
 mulps   xmm10, xmm0
 mulps   xmm10, xmm12
 addps   xmm10, xmm7
 movaps  xmmword, ptr, [rbp, +, 560], xmm6
 mov     rcx, r15
 lea     rdx, [rbp, +, 560]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm1, xmm8
 shufps  xmm1, xmm8, 255
 movaps  xmm0, xmmword, ptr, [rbp, +, 1232]
 movaps  xmmword, ptr, [rbp, +, 768], xmm1
 mulps   xmm0, xmm1
 mulps   xmm0, xmm13
 addps   xmm0, xmm10
 movaps  xmm1, xmmword, ptr, [rbp, +, 1152]
 shufps  xmm1, xmm1, 0
 movaps  xmmword, ptr, [rbp, +, 1152], xmm1
 mulps   xmm1, xmm14
 addps   xmm1, xmm0
 movd    xmm0, ebx
 pshufd  xmm0, xmm0, 0
 movdqa  xmmword, ptr, [rbp, +, 720], xmm0
 mulps   xmm0, xmm15
 addps   xmm0, xmm1
 shr     rbx, 32
 movd    xmm1, ebx
 pshufd  xmm1, xmm1, 0
 movdqa  xmmword, ptr, [rbp, +, 736], xmm1
 mulps   xmm1, xmmword, ptr, [rip, +, __xmm@3f800000000000000000000000000000]
 addps   xmm1, xmm0
 movaps  xmm10, xmmword, ptr, [rbp, +, 1040]
 movlhps xmm10, xmm8
 shufps  xmm10, xmm8, 8
 mulps   xmm10, xmm6
 addps   xmm10, xmm1
 movaps  xmmword, ptr, [rbp, +, 672], xmm11
 movaps  xmm6, xmm11
 movaps  xmm0, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 mulps   xmm6, xmm0
 mulps   xmm9, xmm0
 movaps  xmm14, xmm6
 shufps  xmm14, xmm6, 0
 movaps  xmm7, xmm14
 mulps   xmm7, xmm8
 movaps  xmmword, ptr, [rbp, +, 544], xmm8
 mov     rcx, r15
 lea     rdx, [rbp, +, 544]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmm6
 shufps  xmm11, xmm6, 85
 movaps  xmm12, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm12, xmm11
 mulps   xmm12, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f800000bf800000]
 addps   xmm12, xmm7
 movaps  xmmword, ptr, [rbp, +, 528], xmm8
 mov     rcx, r15
 lea     rdx, [rbp, +, 528]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm13, xmm6
 shufps  xmm13, xmm6, 170
 movaps  xmm15, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm15, xmm13
 mulps   xmm15, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf800000bf800000]
 addps   xmm15, xmm12
 movaps  xmmword, ptr, [rbp, +, 512], xmm8
 mov     rcx, r15
 lea     rdx, [rbp, +, 512]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm6, xmm6, 255
 movaps  xmm7, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm7, xmm6
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f800000bf800000]
 addps   xmm7, xmm15
 mulps   xmm14, xmm10
 movaps  xmmword, ptr, [rbp, +, 496], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 496]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm11, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm11, xmmword, ptr, [rip, +, __xmm@bf8000003f800000bf8000003f800000]
 addps   xmm11, xmm14
 movaps  xmmword, ptr, [rbp, +, 480], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 480]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm13, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm13, xmmword, ptr, [rip, +, __xmm@3f800000bf800000bf8000003f800000]
 addps   xmm13, xmm11
 movaps  xmmword, ptr, [rbp, +, 464], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 464]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm6, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm6, xmmword, ptr, [rip, +, __xmm@bf800000bf8000003f8000003f800000]
 addps   xmm6, xmm13
 movaps  xmm10, xmm9
 shufps  xmm10, xmm9, 0
 mulps   xmm10, xmm8
 mulps   xmm10, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 addps   xmm10, xmm6
 movaps  xmmword, ptr, [rbp, +, 448], xmm8
 mov     rcx, r15
 lea     rdx, [rbp, +, 448]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm6, xmm9
 shufps  xmm6, xmm9, 85
 mulps   xmm6, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm6, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f8000003f800000]
 addps   xmm6, xmm10
 movaps  xmmword, ptr, [rbp, +, 432], xmm8
 mov     rcx, r15
 lea     rdx, [rbp, +, 432]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm10, xmm9
 shufps  xmm10, xmm9, 170
 mulps   xmm10, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm10, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf8000003f800000]
 addps   xmm10, xmm6
 movaps  xmmword, ptr, [rbp, +, 656], xmm8
 movaps  xmmword, ptr, [rbp, +, 416], xmm8
 mov     rcx, r15
 lea     rdx, [rbp, +, 416]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm9, xmm9, 255
 mulps   xmm9, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm9, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f8000003f800000]
 addps   xmm9, xmm10
 movaps  xmmword, ptr, [rbp, +, 1232], xmm7
 movaps  xmmword, ptr, [rbp, +, 1248], xmm9
 mov     rcx, rbp
 mov     rdx, r15
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Ln>::ln
 movaps  xmm0, xmmword, ptr, [rbp]
 movaps  xmm1, xmmword, ptr, [rbp, +, 16]
 movaps  xmmword, ptr, [rbp, +, 1248], xmm1
 movaps  xmmword, ptr, [rbp, +, 1232], xmm0
 lea     rcx, [rbp, -, 32]
 mov     rdx, r15
 call    <geometric_algebra::ppga3d::Line as geometric_algebra::Dual>::dual
 lea     rbx, [r14, +, 64]
 movaps  xmm13, xmmword, ptr, [rbp, -, 16]
 movaps  xmm0, xmm13
 xorps   xmm0, xmmword, ptr, [rip, +, __xmm@80000000800000008000000080000000]
 mulps   xmm0, xmm13
 xorps   xmm1, xmm1
 subss   xmm1, xmm0
 movaps  xmm2, xmm0
 shufps  xmm2, xmm0, 85
 subss   xmm1, xmm2
 movhlps xmm0, xmm0
 subss   xmm1, xmm0
 xorps   xmm0, xmm0
 sqrtss  xmm0, xmm1
 ucomiss xmm0, dword, ptr, [rip, +, __real@00000000]
 jne     .LBB219_58
 jnp     .LBB219_57
.LBB219_58:
 movaps  xmmword, ptr, [rbp, +, 1104], xmm0
 movaps  xmm6, xmmword, ptr, [rbp, -, 32]
 mulps   xmm7, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 movaps  xmmword, ptr, [rbp, +, 400], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 400]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmmword, ptr, [rbp, +, 1184]
 shufps  xmm0, xmm0, 0
 mulps   xmm0, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f8000003f800000]
 movaps  xmmword, ptr, [rbp, +, 1184], xmm0
 movaps  xmmword, ptr, [rbp, +, 384], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 384]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm8, xmmword, ptr, [rbp, +, 1168]
 movaps  xmm0, xmm8
 shufps  xmm0, xmm8, 0
 mulps   xmm0, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf8000003f800000]
 movaps  xmmword, ptr, [rbp, +, 880], xmm0
 movaps  xmmword, ptr, [rbp, +, 368], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 368]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm8, xmm8, 85
 mulps   xmm8, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm8, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f8000003f800000]
 movaps  xmmword, ptr, [rbp, +, 1168], xmm8
 mov     rcx, r15
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::One>::one
 movss   xmm15, dword, ptr, [rip, +, __real@3f800000]
 divss   xmm15, dword, ptr, [rbp, +, 1104]
 shufps  xmm15, xmm15, 0
 mulps   xmm13, xmm15
 mulps   xmm15, xmm6
 movaps  xmm14, xmmword, ptr, [rbp, +, 1232]
 movups  xmm6, xmmword, ptr, [rbp, +, 1236]
 movaps  xmm8, xmmword, ptr, [rbp, +, 1120]
 movaps  xmmword, ptr, [rbp, +, 352], xmm8
 lea     rsi, [rbp, +, 1072]
 mov     rcx, rsi
 lea     rdx, [rbp, +, 352]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm6, xmm6, 0
 mulps   xmm6, xmmword, ptr, [rbp, +, 1072]
 movaps  xmm11, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm6, xmm11
 movups  xmm10, xmmword, ptr, [rbp, +, 1240]
 movaps  xmmword, ptr, [rbp, +, 336], xmm8
 mov     rcx, rsi
 lea     rdx, [rbp, +, 336]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm10, xmm10, 0
 mulps   xmm10, xmmword, ptr, [rbp, +, 1072]
 movaps  xmm12, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm10, xmm12
 addps   xmm10, xmm6
 movups  xmm6, xmmword, ptr, [rbp, +, 1244]
 movaps  xmmword, ptr, [rbp, +, 320], xmm8
 mov     rcx, rsi
 lea     rdx, [rbp, +, 320]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm0, xmm6
 shufps  xmm0, xmm6, 0
 mulps   xmm0, xmmword, ptr, [rbp, +, 1072]
 movaps  xmm5, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm0, xmm5
 addps   xmm0, xmm10
 movaps  xmm1, xmm6
 shufps  xmm1, xmm6, 170
 mulps   xmm1, xmmword, ptr, [rip, +, __xmm@00000000000000003f80000000000000]
 addps   xmm1, xmm0
 shufps  xmm6, xmm6, 255
 mulps   xmm6, xmmword, ptr, [rip, +, __xmm@000000003f8000000000000000000000]
 addps   xmm6, xmm1
 movss   xmm0, dword, ptr, [rbp, +, 1260]
 shufps  xmm0, xmm0, 0
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@3f800000000000000000000000000000]
 addps   xmm0, xmm6
 movaps  xmm12, xmm14
 movaps  xmm10, xmm14
 unpcklpd xmm10, xmmword, ptr, [rbp, +, 1248]
 shufps  xmm10, xmm14, 2
 mulps   xmm10, xmm8
 addps   xmm10, xmm0
 movaps  xmm0, xmm14
 shufps  xmm0, xmm14, 85
 movaps  xmm8, xmm13
 shufps  xmm8, xmm13, 96
 movaps  xmm2, xmm8
 mulps   xmm2, xmm0
 mulps   xmm2, xmmword, ptr, [rip, +, __xmm@bf8000003f80000000000000bf800000]
 movaps  xmm1, xmm14
 shufps  xmm1, xmm14, 170
 movaps  xmm14, xmm13
 shufps  xmm14, xmm13, 25
 movaps  xmm3, xmm14
 mulps   xmm3, xmm1
 mulps   xmm3, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf800000bf800000]
 addps   xmm3, xmm2
 movaps  xmm2, xmm12
 shufps  xmm2, xmm12, 255
 movaps  xmm5, xmm13
 shufps  xmm5, xmm13, 134
 movaps  xmm4, xmm5
 mulps   xmm4, xmm2
 mulps   xmm4, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f800000bf800000]
 addps   xmm4, xmm3
 movaps  xmm6, xmm12
 shufps  xmm6, xmm12, 0
 shufps  xmm13, xmm13, 144
 movaps  xmmword, ptr, [rbp, +, 864], xmm13
 movaps  xmm11, xmm13
 mulps   xmm13, xmm6
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 mulps   xmm13, xmm3
 addps   xmm13, xmm4
 movaps  xmm4, xmm15
 shufps  xmm4, xmm15, 96
 movaps  xmmword, ptr, [rbp, +, 960], xmm4
 mulps   xmm0, xmm4
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm0, xmm3
 movaps  xmm4, xmm15
 shufps  xmm4, xmm15, 25
 movaps  xmmword, ptr, [rbp, +, 976], xmm4
 mulps   xmm1, xmm4
 movaps  xmm4, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm1, xmm4
 addps   xmm1, xmm0
 movaps  xmm0, xmm15
 shufps  xmm0, xmm15, 134
 movaps  xmmword, ptr, [rbp, +, 992], xmm0
 mulps   xmm2, xmm0
 mulps   xmm2, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 addps   xmm2, xmm1
 movaps  xmm0, xmm10
 shufps  xmm0, xmm10, 0
 mulps   xmm0, xmm11
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf80000000000000]
 addps   xmm0, xmm2
 movaps  xmm1, xmm10
 shufps  xmm1, xmm10, 85
 movaps  xmmword, ptr, [rbp, +, 912], xmm8
 mulps   xmm1, xmm8
 mulps   xmm1, xmm3
 addps   xmm1, xmm0
 movaps  xmm0, xmm10
 shufps  xmm0, xmm10, 170
 movaps  xmmword, ptr, [rbp, +, 928], xmm14
 mulps   xmm0, xmm14
 mulps   xmm0, xmm4
 addps   xmm0, xmm1
 movaps  xmm1, xmm10
 shufps  xmm1, xmm10, 255
 movaps  xmmword, ptr, [rbp, +, 944], xmm5
 mulps   xmm1, xmm5
 mulps   xmm1, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 addps   xmm1, xmm0
 shufps  xmm15, xmm15, 144
 movaps  xmmword, ptr, [rbp, +, 896], xmm15
 mulps   xmm6, xmm15
 mulps   xmm6, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 addps   xmm6, xmm1
 movaps  xmm1, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 movaps  xmm0, xmm12
 mulps   xmm0, xmm1
 mulps   xmm10, xmm1
 movaps  xmm12, xmm13
 shufps  xmm12, xmm13, 0
 movaps  xmm8, xmm0
 mulps   xmm8, xmm12
 movaps  xmmword, ptr, [rbp, +, 304], xmm0
 movaps  xmm11, xmm0
 movaps  xmmword, ptr, [rbp, +, 1120], xmm0
 mov     rcx, r15
 lea     rdx, [rbp, +, 304]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm14, xmm13
 shufps  xmm14, xmm13, 85
 movaps  xmm15, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm15, xmm14
 mulps   xmm15, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f800000bf800000]
 addps   xmm15, xmm8
 movaps  xmmword, ptr, [rbp, +, 288], xmm11
 mov     rcx, r15
 lea     rdx, [rbp, +, 288]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmm13
 shufps  xmm11, xmm13, 170
 movaps  xmm8, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm8, xmm11
 mulps   xmm8, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf800000bf800000]
 addps   xmm8, xmm15
 movaps  xmm0, xmmword, ptr, [rbp, +, 1120]
 movaps  xmmword, ptr, [rbp, +, 272], xmm0
 mov     rcx, r15
 lea     rdx, [rbp, +, 272]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm13, xmm13, 255
 movaps  xmm15, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm15, xmm13
 mulps   xmm15, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f800000bf800000]
 addps   xmm15, xmm8
 mulps   xmm12, xmm10
 movaps  xmmword, ptr, [rbp, +, 256], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 256]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm14, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm14, xmmword, ptr, [rip, +, __xmm@bf8000003f800000bf8000003f800000]
 addps   xmm14, xmm12
 movaps  xmmword, ptr, [rbp, +, 240], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 240]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm11, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm11, xmmword, ptr, [rip, +, __xmm@3f800000bf800000bf8000003f800000]
 addps   xmm11, xmm14
 movaps  xmm12, xmmword, ptr, [rbp, +, 864]
 movaps  xmmword, ptr, [rbp, +, 224], xmm10
 mov     rcx, r15
 lea     rdx, [rbp, +, 224]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm13, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm13, xmmword, ptr, [rip, +, __xmm@bf800000bf8000003f8000003f800000]
 addps   xmm13, xmm11
 movaps  xmm8, xmm6
 shufps  xmm8, xmm6, 0
 movaps  xmm0, xmmword, ptr, [rbp, +, 1120]
 mulps   xmm8, xmm0
 mulps   xmm8, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 addps   xmm8, xmm13
 movaps  xmmword, ptr, [rbp, +, 208], xmm0
 movaps  xmm11, xmm0
 mov     rcx, r15
 lea     rdx, [rbp, +, 208]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm10, xmm6
 shufps  xmm10, xmm6, 85
 mulps   xmm10, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm10, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f8000003f800000]
 addps   xmm10, xmm8
 movaps  xmmword, ptr, [rbp, +, 192], xmm11
 mov     rcx, r15
 lea     rdx, [rbp, +, 192]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm8, xmm6
 shufps  xmm8, xmm6, 170
 mulps   xmm8, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm8, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf8000003f800000]
 addps   xmm8, xmm10
 movaps  xmmword, ptr, [rbp, +, 176], xmm11
 mov     rcx, r15
 lea     rdx, [rbp, +, 176]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm6, xmm6, 255
 mulps   xmm6, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm6, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f8000003f800000]
 addps   xmm6, xmm8
 movaps  xmmword, ptr, [rbp, +, 1232], xmm15
 movaps  xmmword, ptr, [rbp, +, 1248], xmm6
 lea     rcx, [rbp, -, 64]
 mov     rdx, r15
 call    <geometric_algebra::ppga3d::Motor as core::convert::Into<geometric_algebra::ppga3d::Line>>::into
 movaps  xmm15, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 mulps   xmm9, xmm15
 addps   xmm9, xmmword, ptr, [rbp, +, 1184]
 addps   xmm9, xmmword, ptr, [rbp, +, 880]
 addps   xmm9, xmmword, ptr, [rbp, +, 1168]
 movaps  xmm0, xmm7
 shufps  xmm0, xmm7, 85
 movaps  xmm2, xmm0
 movaps  xmm11, xmmword, ptr, [rbp, +, 912]
 mulps   xmm2, xmm11
 mulps   xmm2, xmmword, ptr, [rip, +, __xmm@bf8000003f80000000000000bf800000]
 movaps  xmm1, xmm7
 shufps  xmm1, xmm7, 170
 movaps  xmm3, xmm1
 movaps  xmm13, xmmword, ptr, [rbp, +, 928]
 mulps   xmm3, xmm13
 mulps   xmm3, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf800000bf800000]
 addps   xmm3, xmm2
 movaps  xmm2, xmm7
 shufps  xmm2, xmm7, 255
 movaps  xmm4, xmm2
 movaps  xmm14, xmmword, ptr, [rbp, +, 944]
 mulps   xmm4, xmm14
 mulps   xmm4, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f800000bf800000]
 addps   xmm4, xmm3
 movaps  xmm6, xmm7
 shufps  xmm6, xmm7, 0
 movaps  xmm10, xmm6
 mulps   xmm10, xmm12
 movaps  xmm8, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 mulps   xmm10, xmm8
 addps   xmm10, xmm4
 mulps   xmm0, xmmword, ptr, [rbp, +, 960]
 mulps   xmm1, xmmword, ptr, [rbp, +, 976]
 movaps  xmm3, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm0, xmm3
 movaps  xmm4, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm1, xmm4
 addps   xmm1, xmm0
 mulps   xmm2, xmmword, ptr, [rbp, +, 992]
 movaps  xmm5, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm2, xmm5
 addps   xmm2, xmm1
 movaps  xmm0, xmm9
 shufps  xmm0, xmm9, 0
 mulps   xmm0, xmm12
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf80000000000000]
 addps   xmm0, xmm2
 movaps  xmm1, xmm9
 shufps  xmm1, xmm9, 85
 mulps   xmm1, xmm11
 mulps   xmm1, xmm3
 addps   xmm1, xmm0
 movaps  xmm0, xmm9
 shufps  xmm0, xmm9, 170
 mulps   xmm0, xmm13
 mulps   xmm0, xmm4
 addps   xmm0, xmm1
 movaps  xmm1, xmm9
 shufps  xmm1, xmm9, 255
 mulps   xmm1, xmm14
 mulps   xmm1, xmm5
 addps   xmm1, xmm0
 mulps   xmm6, xmmword, ptr, [rbp, +, 896]
 mulps   xmm6, xmm8
 addps   xmm6, xmm1
 mulps   xmm7, xmm15
 mulps   xmm9, xmm15
 movaps  xmm12, xmm10
 shufps  xmm12, xmm10, 0
 movaps  xmm8, xmm7
 mulps   xmm8, xmm12
 movaps  xmmword, ptr, [rbp, +, 160], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 160]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm11, xmm10
 shufps  xmm11, xmm10, 85
 movaps  xmm14, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm14, xmm11
 mulps   xmm14, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f800000bf800000]
 addps   xmm14, xmm8
 movaps  xmmword, ptr, [rbp, +, 144], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 144]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm13, xmm10
 shufps  xmm13, xmm10, 170
 movaps  xmm15, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm15, xmm13
 mulps   xmm15, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf800000bf800000]
 addps   xmm15, xmm14
 movaps  xmmword, ptr, [rbp, +, 128], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 128]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm10, xmm10, 255
 movaps  xmm8, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm8, xmm10
 mulps   xmm8, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f800000bf800000]
 addps   xmm8, xmm15
 mulps   xmm12, xmm9
 movaps  xmmword, ptr, [rbp, +, 112], xmm9
 mov     rcx, r15
 lea     rdx, [rbp, +, 112]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm11, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm11, xmmword, ptr, [rip, +, __xmm@bf8000003f800000bf8000003f800000]
 addps   xmm11, xmm12
 movaps  xmmword, ptr, [rbp, +, 96], xmm9
 mov     rcx, r15
 lea     rdx, [rbp, +, 96]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm13, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm13, xmmword, ptr, [rip, +, __xmm@3f800000bf800000bf8000003f800000]
 addps   xmm13, xmm11
 movaps  xmmword, ptr, [rbp, +, 80], xmm9
 mov     rcx, r15
 lea     rdx, [rbp, +, 80]
 call    core::core_arch::x86::avx::_mm_permute_ps
 mulps   xmm10, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm10, xmmword, ptr, [rip, +, __xmm@bf800000bf8000003f8000003f800000]
 addps   xmm10, xmm13
 movaps  xmm9, xmm6
 shufps  xmm9, xmm6, 0
 mulps   xmm9, xmm7
 mulps   xmm9, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 addps   xmm9, xmm10
 movaps  xmmword, ptr, [rbp, +, 64], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 64]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm10, xmm6
 shufps  xmm10, xmm6, 85
 mulps   xmm10, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm10, xmmword, ptr, [rip, +, __xmm@bf8000003f8000003f8000003f800000]
 addps   xmm10, xmm9
 movaps  xmmword, ptr, [rbp, +, 48], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 48]
 call    core::core_arch::x86::avx::_mm_permute_ps
 movaps  xmm9, xmm6
 shufps  xmm9, xmm6, 170
 mulps   xmm9, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm9, xmmword, ptr, [rip, +, __xmm@3f8000003f800000bf8000003f800000]
 addps   xmm9, xmm10
 movaps  xmmword, ptr, [rbp, +, 32], xmm7
 mov     rcx, r15
 lea     rdx, [rbp, +, 32]
 call    core::core_arch::x86::avx::_mm_permute_ps
 shufps  xmm6, xmm6, 255
 mulps   xmm6, xmmword, ptr, [rbp, +, 1232]
 mulps   xmm6, xmmword, ptr, [rip, +, __xmm@3f800000bf8000003f8000003f800000]
 addps   xmm6, xmm9
 movaps  xmmword, ptr, [rbp, +, 1232], xmm8
 movaps  xmmword, ptr, [rbp, +, 1248], xmm6
 lea     rcx, [rbp, +, 1072]
 mov     rdx, r15
 call    <geometric_algebra::ppga3d::Motor as core::convert::Into<geometric_algebra::ppga3d::Line>>::into
 movaps  xmm2, xmmword, ptr, [rbp, -, 48]
 movaps  xmm1, xmm2
 movaps  xmm5, xmmword, ptr, [rbp, +, 1024]
 divps   xmm1, xmm5
 movaps  xmm0, xmmword, ptr, [rbp, +, 1088]
 movaps  xmm3, xmm2
 unpcklps xmm3, xmm0
 movaps  xmm4, xmm0
 shufps  xmm4, xmm2, 17
 shufps  xmm4, xmm2, 226
 unpckhps xmm2, xmm0
 divps   xmm0, xmm5
 movaps  xmm5, xmm1
 unpcklps xmm5, xmm0
 mulps   xmm3, xmm5
 movaps  xmm5, xmm0
 shufps  xmm5, xmm1, 17
 shufps  xmm5, xmm1, 226
 mulps   xmm4, xmm5
 addps   xmm4, xmm3
 movaps  xmm3, xmm1
 unpckhps xmm3, xmm0
 mulps   xmm2, xmm3
 addps   xmm2, xmm4
 movaps  xmm3, xmmword, ptr, [rbp, -, 64]
 movaps  xmm4, xmm3
 movaps  xmm8, xmmword, ptr, [rbp, +, 1008]
 divps   xmm4, xmm8
 movaps  xmm5, xmmword, ptr, [rbp, +, 1072]
 movaps  xmm6, xmm3
 unpcklps xmm6, xmm5
 movaps  xmm7, xmm5
 shufps  xmm7, xmm3, 17
 shufps  xmm7, xmm3, 226
 unpckhps xmm3, xmm5
 divps   xmm5, xmm8
 movaps  xmm8, xmm4
 unpcklps xmm8, xmm5
 mulps   xmm6, xmm8
 addps   xmm6, xmm2
 movaps  xmm2, xmm5
 shufps  xmm2, xmm4, 17
 shufps  xmm2, xmm4, 226
 mulps   xmm7, xmm2
 addps   xmm7, xmm6
 movaps  xmm2, xmm4
 unpckhps xmm2, xmm5
 mulps   xmm3, xmm2
 addps   xmm3, xmm7
 movaps  xmm2, xmm3
 shufps  xmm2, xmm3, 85
 addss   xmm2, xmm3
 movss   xmm3, dword, ptr, [r14, +, 60]
 divss   xmm3, dword, ptr, [rbp, +, 1228]
 addss   xmm3, xmm2
 movaps  xmm2, xmmword, ptr, [rbp, +, 1104]
 divss   xmm2, xmm3
 shufps  xmm2, xmm2, 0
 movaps  xmm3, xmm2
 xorps   xmm3, xmmword, ptr, [rip, +, __xmm@80000000800000008000000080000000]
 mulps   xmm1, xmm3
 mulps   xmm3, xmm4
 mulps   xmm0, xmm2
 movaps  xmmword, ptr, [rbp, +, 1168], xmm0
 mulps   xmm2, xmm5
 movaps  xmmword, ptr, [rbp, +, 1104], xmm2
 movaps  xmm4, xmm3
 shufps  xmm4, xmm3, 96
 movaps  xmmword, ptr, [rbp, +, 1184], xmm4
 movaps  xmm0, xmmword, ptr, [rbp, +, 784]
 movaps  xmm2, xmm0
 mulps   xmm2, xmm4
 movaps  xmm12, xmmword, ptr, [rip, +, __xmm@bf8000003f80000000000000bf800000]
 mulps   xmm2, xmm12
 movaps  xmm5, xmm3
 shufps  xmm5, xmm3, 25
 movaps  xmm9, xmmword, ptr, [rbp, +, 832]
 movaps  xmm7, xmm9
 mulps   xmm7, xmm5
 movaps  xmm13, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf800000bf800000]
 mulps   xmm7, xmm13
 addps   xmm7, xmm2
 movaps  xmm6, xmm3
 shufps  xmm6, xmm3, 134
 movaps  xmm11, xmmword, ptr, [rbp, +, 848]
 movaps  xmm8, xmm11
 mulps   xmm8, xmm6
 movaps  xmm14, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f800000bf800000]
 mulps   xmm8, xmm14
 addps   xmm8, xmm7
 movaps  xmm4, xmmword, ptr, [rbp, +, 672]
 shufps  xmm4, xmm4, 0
 shufps  xmm3, xmm3, 144
 movaps  xmm2, xmm4
 mulps   xmm2, xmm3
 movaps  xmm12, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 mulps   xmm2, xmm12
 addps   xmm2, xmm8
 movaps  xmm7, xmm1
 shufps  xmm7, xmm1, 96
 mulps   xmm7, xmm0
 movaps  xmm8, xmm1
 shufps  xmm8, xmm1, 25
 mulps   xmm8, xmm9
 movaps  xmm15, xmmword, ptr, [rip, +, __xmm@bf8000003f800000000000003f800000]
 mulps   xmm7, xmm15
 movaps  xmm10, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm8, xmm10
 addps   xmm8, xmm7
 movaps  xmm7, xmm1
 shufps  xmm7, xmm1, 134
 mulps   xmm7, xmm11
 movaps  xmm11, xmmword, ptr, [rip, +, __xmm@00000000bf8000003f8000003f800000]
 mulps   xmm7, xmm11
 addps   xmm7, xmm8
 movaps  xmm0, xmmword, ptr, [rbp, +, 1056]
 shufps  xmm0, xmm0, 0
 mulps   xmm0, xmm3
 movaps  xmm8, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf80000000000000]
 mulps   xmm0, xmm8
 addps   xmm0, xmm7
 movaps  xmm3, xmmword, ptr, [rbp, +, 688]
 mulps   xmm3, xmmword, ptr, [rbp, +, 1184]
 mulps   xmm3, xmm15
 addps   xmm3, xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 800]
 mulps   xmm0, xmm5
 mulps   xmm0, xmm10
 addps   xmm0, xmm3
 movaps  xmm3, xmmword, ptr, [rbp, +, 816]
 mulps   xmm3, xmm6
 mulps   xmm3, xmm11
 addps   xmm3, xmm0
 shufps  xmm1, xmm1, 144
 mulps   xmm1, xmm4
 movaps  xmm9, xmm12
 mulps   xmm1, xmm12
 addps   xmm1, xmm3
 movaps  xmm10, xmmword, ptr, [rip, +, __xmm@3f0000003f0000003f0000003f000000]
 mulps   xmm2, xmm10
 mulps   xmm1, xmm10
 movaps  xmm3, xmmword, ptr, [r13]
 subps   xmm3, xmm2
 movaps  xmm2, xmmword, ptr, [r13, +, 16]
 subps   xmm2, xmm1
 movaps  xmmword, ptr, [r13], xmm3
 movaps  xmmword, ptr, [r13, +, 16], xmm2
 movaps  xmm8, xmmword, ptr, [rbp, +, 1104]
 movaps  xmm2, xmm8
 shufps  xmm2, xmm8, 96
 movaps  xmm0, xmmword, ptr, [rbp, +, 704]
 movaps  xmm1, xmm0
 mulps   xmm1, xmm2
 mulps   xmm1, xmmword, ptr, [rip, +, __xmm@bf8000003f80000000000000bf800000]
 movaps  xmm3, xmm8
 shufps  xmm3, xmm8, 25
 movaps  xmm7, xmmword, ptr, [rbp, +, 752]
 movaps  xmm5, xmm7
 mulps   xmm5, xmm3
 mulps   xmm5, xmm13
 addps   xmm5, xmm1
 movaps  xmm4, xmm8
 shufps  xmm4, xmm8, 134
 movaps  xmm13, xmmword, ptr, [rbp, +, 768]
 movaps  xmm6, xmm13
 mulps   xmm6, xmm4
 mulps   xmm6, xmm14
 addps   xmm6, xmm5
 movaps  xmm12, xmmword, ptr, [rbp, +, 656]
 shufps  xmm12, xmm12, 0
 shufps  xmm8, xmm8, 144
 movaps  xmm1, xmm12
 mulps   xmm1, xmm8
 mulps   xmm1, xmm9
 addps   xmm1, xmm6
 movaps  xmm14, xmmword, ptr, [rbp, +, 1168]
 movaps  xmm5, xmm14
 shufps  xmm5, xmm14, 96
 mulps   xmm5, xmm0
 movaps  xmm6, xmm14
 shufps  xmm6, xmm14, 25
 mulps   xmm6, xmm7
 mulps   xmm5, xmm15
 movaps  xmm9, xmmword, ptr, [rip, +, __xmm@3f80000000000000bf8000003f800000]
 mulps   xmm6, xmm9
 addps   xmm6, xmm5
 movaps  xmm5, xmm14
 movaps  xmm7, xmm14
 shufps  xmm5, xmm5, 134
 mulps   xmm5, xmm13
 mulps   xmm5, xmm11
 addps   xmm5, xmm6
 movaps  xmm6, xmmword, ptr, [rbp, +, 1040]
 shufps  xmm6, xmm6, 0
 mulps   xmm6, xmm8
 mulps   xmm6, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf80000000000000]
 addps   xmm6, xmm5
 movaps  xmm5, xmmword, ptr, [rbp, +, 1152]
 mulps   xmm5, xmm2
 mulps   xmm5, xmm15
 addps   xmm5, xmm6
 movaps  xmm0, xmmword, ptr, [rbp, +, 720]
 mulps   xmm0, xmm3
 mulps   xmm0, xmm9
 addps   xmm0, xmm5
 movaps  xmm2, xmmword, ptr, [rbp, +, 736]
 mulps   xmm2, xmm4
 add     r12, rdi
 mulps   xmm2, xmm11
 addps   xmm2, xmm0
 movaps  xmm0, xmm14
 shufps  xmm0, xmm14, 144
 mulps   xmm0, xmm12
 mulps   xmm0, xmmword, ptr, [rip, +, __xmm@3f8000003f8000003f80000000000000]
 addps   xmm0, xmm2
 mulps   xmm1, xmm10
 mulps   xmm0, xmm10
 movaps  xmm2, xmmword, ptr, [r12]
 subps   xmm2, xmm1
 movaps  xmm1, xmmword, ptr, [r12, +, 16]
 subps   xmm1, xmm0
 movaps  xmmword, ptr, [r12], xmm2
 movaps  xmmword, ptr, [r12, +, 16], xmm1
 movaps  xmm0, xmmword, ptr, [r13]
 movaps  xmm1, xmm0
 movaps  xmm5, xmmword, ptr, [rip, +, __xmm@bf800000bf800000bf8000003f800000]
 mulps   xmm1, xmm5
 mulps   xmm1, xmm0
 movaps  xmm2, xmm1
 shufps  xmm2, xmm1, 255
 movaps  xmm3, xmm1
 unpckhpd xmm3, xmm1
 movaps  xmm4, xmm1
 shufps  xmm4, xmm1, 85
 subss   xmm1, xmm4
 subss   xmm1, xmm3
 subss   xmm1, xmm2
 sqrtss  xmm1, xmm1
 movaps  xmm2, xmm1
 mulss   xmm2, xmm1
 movss   xmm6, dword, ptr, [rip, +, __real@3f800000]
 movaps  xmm3, xmm6
 divss   xmm3, xmm2
 mulss   xmm3, xmm1
 shufps  xmm3, xmm3, 0
 mulps   xmm0, xmm3
 mulps   xmm3, xmmword, ptr, [r13, +, 16]
 movaps  xmmword, ptr, [r13], xmm0
 movaps  xmmword, ptr, [r13, +, 16], xmm3
 movaps  xmm0, xmmword, ptr, [r12]
 movaps  xmm1, xmm0
 mulps   xmm1, xmm5
 mulps   xmm1, xmm0
 movaps  xmm2, xmm1
 movaps  xmm3, xmm1
 movaps  xmm4, xmm1
 shufps  xmm4, xmm1, 85
 subss   xmm1, xmm4
 shufps  xmm2, xmm2, 255
 movhlps xmm3, xmm3
 subss   xmm1, xmm3
 subss   xmm1, xmm2
 sqrtss  xmm1, xmm1
 movaps  xmm2, xmm1
 mulss   xmm2, xmm1
 movaps  xmm3, xmm6
 divss   xmm3, xmm2
 mulss   xmm3, xmm1
 shufps  xmm3, xmm3, 0
 mulps   xmm0, xmm3
 mulps   xmm3, xmmword, ptr, [r12, +, 16]
 movaps  xmmword, ptr, [r12], xmm0
 movaps  xmmword, ptr, [r12, +, 16], xmm3
 jmp     .LBB219_57
.LBB219_45:
 mov     rcx, qword, ptr, [rbp, +, 1216]
 mov     rdx, qword, ptr, [rbp, +, 1144]
 movaps  xmm6, xmmword, ptr, [rbp, +, 1280]
 movaps  xmm7, xmmword, ptr, [rbp, +, 1296]
 movaps  xmm8, xmmword, ptr, [rbp, +, 1312]
 movaps  xmm9, xmmword, ptr, [rbp, +, 1328]
 movaps  xmm10, xmmword, ptr, [rbp, +, 1344]
 movaps  xmm11, xmmword, ptr, [rbp, +, 1360]
 movaps  xmm12, xmmword, ptr, [rbp, +, 1376]
 movaps  xmm13, xmmword, ptr, [rbp, +, 1392]
 movaps  xmm14, xmmword, ptr, [rbp, +, 1408]
 movaps  xmm15, xmmword, ptr, [rbp, +, 1424]
 add     rsp, 1576
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
 mov     rcx, r12
 mov     rdx, rbx
 call    core::panicking::panic_bounds_check
.LBB219_53:
 ud2
.LBB219_64:
 lea     rax, [rip, +, __unnamed_23]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_24]
 lea     r9, [rip, +, __unnamed_25]
 lea     r8, [rbp, +, 1232]
 mov     edx, 16
 call    core::result::unwrap_failed
 ud2
.LBB219_63:
 lea     rax, [rip, +, __unnamed_112]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_101]
 mov     edx, 24
 mov     r9d, 24
 call    core::str::slice_error_fail
 ud2
.LBB219_62:
 lea     rax, [rip, +, __unnamed_114]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_115]
 mov     edx, 60
 mov     r9d, 60
 call    core::str::slice_error_fail
 ud2
