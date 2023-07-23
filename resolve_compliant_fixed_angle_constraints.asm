xpbd_rigid_bodies::resolve_compliant_fixed_angle_constraints:
 push    rbp
 push    r15
 push    r14
 push    r13
 push    r12
 push    rsi
 push    rdi
 push    rbx
 sub     rsp, 904
 lea     rbp, [rsp, +, 128]
 movaps  xmmword, ptr, [rbp, +, 752], xmm15
 movaps  xmmword, ptr, [rbp, +, 736], xmm14
 movaps  xmmword, ptr, [rbp, +, 720], xmm13
 movaps  xmmword, ptr, [rbp, +, 704], xmm12
 movaps  xmmword, ptr, [rbp, +, 688], xmm11
 movaps  xmmword, ptr, [rbp, +, 672], xmm10
 movaps  xmmword, ptr, [rbp, +, 656], xmm9
 movaps  xmmword, ptr, [rbp, +, 640], xmm8
 movaps  xmmword, ptr, [rbp, +, 624], xmm7
 movaps  xmmword, ptr, [rbp, +, 608], xmm6
 mov     qword, ptr, [rbp, +, 600], -2
 mov     qword, ptr, [rbp, +, 512], r9
 mov     rdi, r8
 mov     r13, rdx
 mov     r15, rcx
 movss   xmm9, dword, ptr, [rbp, +, 888]
 mov     rax, qword, ptr, [rbp, +, 880]
 mov     qword, ptr, [rbp, +, 496], rax
 call    puffin::are_scopes_on
 test    al, al
 je      .LBB202_1
 lea     rbx, [rip, +, __unnamed_124]
 mov     esi, 60
 mov     edx, 60
 mov     rcx, rbx
 call    core::str::<impl str>::rfind
 cmp     rax, 1
 jne     .LBB202_16
 mov     r9, rdx
 test    rdx, rdx
 je      .LBB202_8
 cmp     r9, 60
 jae     .LBB202_5
 lea     rax, [rip, +, __unnamed_124]
 cmp     byte, ptr, [r9, +, rax], -65
 jg      .LBB202_8
.LBB202_6:
 lea     rax, [rip, +, __unnamed_122]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_124]
 mov     edx, 60
 xor     r8d, r8d
 call    core::str::slice_error_fail
 ud2
.LBB202_1:
 xor     eax, eax
 mov     qword, ptr, [rbp, +, 504], rax
 jmp     .LBB202_43
.LBB202_5:
 jne     .LBB202_6
.LBB202_8:
 lea     rbx, [rip, +, __unnamed_124]
 mov     rcx, rbx
 mov     rdx, r9
 call    core::str::<impl str>::rfind
 cmp     rax, 1
 jne     .LBB202_16
 mov     r8, rdx
 add     r8, 2
 je      .LBB202_15
 cmp     r8, 60
 jae     .LBB202_11
 lea     rax, [rip, +, __unnamed_124]
 cmp     byte, ptr, [rdx, +, rax, +, 2], -65
 jle     .LBB202_94
 mov     esi, 58
 sub     rsi, rdx
 jmp     .LBB202_15
.LBB202_11:
 jne     .LBB202_94
 xor     esi, esi
.LBB202_15:
 lea     rbx, [rip, +, __unnamed_124]
 add     rbx, r8
.LBB202_16:
 lea     rax, [rip, +, __unnamed_101+24]
 lea     r14, [rip, +, __unnamed_101]
.LBB202_17:
 movzx   ecx, byte, ptr, [rax, -, 1]
 test    cl, cl
 js      .LBB202_21
 dec     rax
 movzx   ecx, cl
 cmp     ecx, 92
 jne     .LBB202_25
 jmp     .LBB202_26
.LBB202_21:
 movzx   edx, byte, ptr, [rax, -, 2]
 cmp     dl, -64
 jge     .LBB202_22
 movzx   r8d, byte, ptr, [rax, -, 3]
 cmp     r8b, -64
 jge     .LBB202_31
 movzx   r9d, byte, ptr, [rax, -, 4]
 add     rax, -4
 and     r9d, 7
 shl     r9d, 6
 and     r8d, 63
 or      r8d, r9d
 jmp     .LBB202_32
.LBB202_22:
 add     rax, -2
 and     edx, 31
 jmp     .LBB202_23
.LBB202_31:
 add     rax, -3
 and     r8d, 15
.LBB202_32:
 shl     r8d, 6
 and     edx, 63
 or      edx, r8d
.LBB202_23:
 shl     edx, 6
 and     cl, 63
 movzx   ecx, cl
 or      ecx, edx
 cmp     ecx, 92
 je      .LBB202_26
.LBB202_25:
 cmp     ecx, 47
 je      .LBB202_26
 cmp     rax, r14
 jne     .LBB202_17
 mov     r12d, 24
 call    puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY{{tls.shim}}
 cmp     qword, ptr, [rax], 0
 jne     .LBB202_38
.LBB202_39:
 mov     rcx, rax
 xor     edx, edx
 call    std::sys::common::thread_local::fast_local::Key<T>::try_initialize
 test    rax, rax
 jne     .LBB202_40
 lea     rax, [rip, +, __unnamed_26]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_27]
 lea     r9, [rip, +, __unnamed_28]
 lea     r8, [rbp, +, 560]
 mov     edx, 70
 call    core::result::unwrap_failed
 ud2
.LBB202_26:
 lea     r14, [rip, +, __unnamed_101]
 sub     rax, r14
 mov     r8, rax
 mov     r12d, 24
 inc     r8
 je      .LBB202_36
 cmp     r8, 24
 jae     .LBB202_28
 cmp     byte, ptr, [r8, +, r14], -65
 jle     .LBB202_95
 mov     r12d, 23
 sub     r12, rax
 jmp     .LBB202_36
.LBB202_28:
 jne     .LBB202_95
 xor     r12d, r12d
.LBB202_36:
 add     r14, r8
 call    puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY{{tls.shim}}
 cmp     qword, ptr, [rax], 0
 je      .LBB202_39
.LBB202_38:
 add     rax, 8
.LBB202_40:
 cmp     qword, ptr, [rax], 0
 jne     .LBB202_96
 mov     qword, ptr, [rax], -1
 mov     qword, ptr, [rbp, +, 520], rax
 lea     rcx, [rax, +, 8]
 lea     rax, [rip, +, __unnamed_16]
 mov     qword, ptr, [rsp, +, 40], rax
 mov     qword, ptr, [rsp, +, 32], r12
 mov     qword, ptr, [rsp, +, 48], 0
 mov     rdx, rbx
 mov     r8, rsi
 mov     r9, r14
 call    puffin::ThreadProfiler::begin_scope
 mov     qword, ptr, [rbp, +, 456], rax
 mov     rax, qword, ptr, [rbp, +, 520]
 inc     qword, ptr, [rax]
 mov     eax, 1
 mov     qword, ptr, [rbp, +, 504], rax
.LBB202_43:
 mov     rdx, qword, ptr, [rbp, +, 512]
 shl     r13, 6
 add     r13, r15
 mov     qword, ptr, [rbp, +, 520], r13
 mov     rax, qword, ptr, [rbp, +, 496]
 add     rax, 16
 mov     qword, ptr, [rbp, +, 448], rax
 mulss   xmm9, xmm9
 movaps  xmm10, xmmword, ptr, [rip, +, __xmm@0000000000000000bf000000bf000000]
 movss   xmm11, dword, ptr, [rip, +, __real@bf000000]
 lea     r13, [rbp, +, 560]
 lea     r12, [rbp, +, 528]
 xorps   xmm12, xmm12
 movss   xmm6, dword, ptr, [rip, +, __real@3f000000]
 jmp     .LBB202_44
.LBB202_93:
 mov     r15, r14
 mov     rdx, qword, ptr, [rbp, +, 512]
.LBB202_44:
 cmp     r15, qword, ptr, [rbp, +, 520]
 je      .LBB202_45
 mov     rsi, qword, ptr, [r15, +, 16]
 cmp     rsi, rdx
 jae     .LBB202_52
 mov     rbx, qword, ptr, [r15, +, 24]
 shl     rsi, 5
 movaps  xmm0, xmmword, ptr, [rdi, +, rsi]
 movaps  xmm1, xmmword, ptr, [rdi, +, rsi, +, 16]
 movaps  xmmword, ptr, [rbp, +, 112], xmm1
 movaps  xmmword, ptr, [rbp, +, 96], xmm0
 cmp     rbx, rdx
 jae     .LBB202_51
 add     rsi, rdi
 shl     rbx, 5
 movaps  xmm0, xmmword, ptr, [rdi, +, rbx]
 movaps  xmm1, xmmword, ptr, [rdi, +, rbx, +, 16]
 movaps  xmmword, ptr, [rbp, +, 368], xmm1
 movaps  xmmword, ptr, [rbp, +, 352], xmm0
 movsd   xmm8, qword, ptr, [r15, +, 32]
 movsd   xmm13, qword, ptr, [r15, +, 44]
 mulps   xmm8, xmm10
 movss   xmm15, dword, ptr, [r15, +, 40]
 mulss   xmm15, xmm11
 movss   xmm14, dword, ptr, [r15, +, 52]
 mulss   xmm14, xmm11
 mov     dword, ptr, [rbp, +, 528], 1065353216
 movlps  qword, ptr, [rbp, +, 532], xmm8
 movss   dword, ptr, [rbp, +, 540], xmm15
 movaps  xmm0, xmmword, ptr, [rsi]
 movaps  xmm1, xmmword, ptr, [rsi, +, 16]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 64]
 mov     rdx, r13
 mov     r8, r12
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::GeometricProduct<geometric_algebra::ppga3d::Translator>>::geometric_product
 mulps   xmm13, xmm10
 mov     dword, ptr, [rbp, +, 528], 1065353216
 movlps  qword, ptr, [rbp, +, 532], xmm13
 movss   dword, ptr, [rbp, +, 540], xmm14
 movaps  xmm0, xmmword, ptr, [rbp, +, 352]
 movaps  xmm1, xmmword, ptr, [rbp, +, 368]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 32]
 mov     rdx, r13
 mov     r8, r12
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::GeometricProduct<geometric_algebra::ppga3d::Translator>>::geometric_product
 movaps  xmm0, xmmword, ptr, [rbp, +, 64]
 movaps  xmm1, xmmword, ptr, [rbp, +, 80]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 mov     rcx, r12
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Reversal>::reversal
 movaps  xmm0, xmmword, ptr, [rbp, +, 32]
 movaps  xmm1, xmmword, ptr, [rbp, +, 48]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 256]
 mov     rdx, r12
 mov     r8, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::GeometricProduct<geometric_algebra::ppga3d::Motor>>::geometric_product
 movaps  xmm0, xmmword, ptr, [rbp, +, 256]
 movaps  xmm1, xmmword, ptr, [rbp, +, 272]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 mov     rcx, rbp
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Ln>::ln
 movaps  xmm0, xmmword, ptr, [rbp]
 movaps  xmm1, xmmword, ptr, [rbp, +, 16]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 224]
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Line as geometric_algebra::Dual>::dual
 movaps  xmm0, xmmword, ptr, [rbp, +, 224]
 movaps  xmm1, xmmword, ptr, [rbp, +, 240]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 mov     rcx, r13
 call    <geometric_algebra::ppga3d::Line as geometric_algebra::Magnitude>::magnitude
 mov     ecx, eax
 call    geometric_algebra::ppga3d::<impl core::convert::From<geometric_algebra::ppga3d::Scalar> for f32>::from
 movaps  xmm7, xmm0
 lea     r14, [r15, +, 64]
 ucomiss xmm0, xmm12
 jne     .LBB202_63
 jnp     .LBB202_93
.LBB202_63:
 movaps  xmm0, xmmword, ptr, [rbp, +, 224]
 movaps  xmm1, xmmword, ptr, [rbp, +, 240]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 movss   xmm2, dword, ptr, [rip, +, __real@3f800000]
 divss   xmm2, xmm7
 lea     rcx, [rbp, +, 192]
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Line as geometric_algebra::Scale>::scale
 mov     dword, ptr, [rbp, +, 464], 1065353216
 movlps  qword, ptr, [rbp, +, 468], xmm13
 movss   dword, ptr, [rbp, +, 476], xmm14
 movaps  xmm0, xmmword, ptr, [rbp, +, 256]
 movaps  xmm1, xmmword, ptr, [rbp, +, 272]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 mov     rcx, r12
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Reversal>::reversal
 lea     rcx, [rbp, -, 32]
 lea     rdx, [rbp, +, 464]
 mov     r8, r12
 call    <geometric_algebra::ppga3d::Translator as geometric_algebra::GeometricProduct<geometric_algebra::ppga3d::Motor>>::geometric_product
 mov     rcx, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::One>::one
 mov     dword, ptr, [rbp, +, 528], 1065353216
 movlps  qword, ptr, [rbp, +, 532], xmm8
 movss   dword, ptr, [rbp, +, 540], xmm15
 lea     rcx, [rbp, -, 64]
 mov     rdx, r13
 mov     r8, r12
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::GeometricProduct<geometric_algebra::ppga3d::Translator>>::geometric_product
 movaps  xmm0, xmmword, ptr, [rbp, -, 64]
 movaps  xmm1, xmmword, ptr, [rbp, -, 48]
 movaps  xmmword, ptr, [rbp, +, 544], xmm1
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 192]
 movaps  xmm1, xmmword, ptr, [rbp, +, 208]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 320]
 mov     rdx, r12
 mov     r8, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Transformation<geometric_algebra::ppga3d::Line>>::transformation
 movaps  xmm0, xmmword, ptr, [rbp, -, 32]
 movaps  xmm1, xmmword, ptr, [rbp, -, 16]
 movaps  xmmword, ptr, [rbp, +, 544], xmm1
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 192]
 movaps  xmm1, xmmword, ptr, [rbp, +, 208]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 288]
 mov     rdx, r12
 mov     r8, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Transformation<geometric_algebra::ppga3d::Line>>::transformation
 movaps  xmm0, xmmword, ptr, [rbp, +, 320]
 movaps  xmm1, xmmword, ptr, [rbp, +, 336]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rax, [rbp, +, 576]
 movaps  xmm0, xmmword, ptr, [rax]
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 lea     rcx, [rbp, +, 416]
 mov     rdx, r12
 mov     r8, qword, ptr, [rbp, +, 496]
 call    <geometric_algebra::simd::Simd32x3 as core::ops::arith::Div>::div
 movaps  xmm0, xmmword, ptr, [rbp, +, 320]
 movaps  xmmword, ptr, [rbp, +, 464], xmm0
 mov     rax, qword, ptr, [rbp, +, 448]
 movaps  xmm0, xmmword, ptr, [rax]
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 lea     rcx, [rbp, +, 384]
 lea     rdx, [rbp, +, 464]
 mov     r8, r12
 call    <geometric_algebra::simd::Simd32x3 as core::ops::arith::Div>::div
 lea     rcx, [rbp, +, 160]
 lea     rdx, [rbp, +, 416]
 lea     r8, [rbp, +, 384]
 call    geometric_algebra::ppga3d::Line::from_groups
 movaps  xmm0, xmmword, ptr, [rbp, +, 288]
 movaps  xmm1, xmmword, ptr, [rbp, +, 304]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rax, [rbp, +, 576]
 movaps  xmm0, xmmword, ptr, [rax]
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 lea     rcx, [rbp, +, 416]
 mov     rdx, r12
 mov     r8, qword, ptr, [rbp, +, 496]
 call    <geometric_algebra::simd::Simd32x3 as core::ops::arith::Div>::div
 movaps  xmm0, xmmword, ptr, [rbp, +, 288]
 movaps  xmmword, ptr, [rbp, +, 464], xmm0
 mov     rax, qword, ptr, [rbp, +, 448]
 movaps  xmm0, xmmword, ptr, [rax]
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 lea     rcx, [rbp, +, 384]
 lea     rdx, [rbp, +, 464]
 mov     r8, r12
 call    <geometric_algebra::simd::Simd32x3 as core::ops::arith::Div>::div
 lea     rcx, [rbp, +, 128]
 lea     rdx, [rbp, +, 416]
 lea     r8, [rbp, +, 384]
 call    geometric_algebra::ppga3d::Line::from_groups
 movaps  xmm0, xmmword, ptr, [rbp, +, 160]
 movaps  xmm1, xmmword, ptr, [rbp, +, 176]
 movaps  xmmword, ptr, [rbp, +, 544], xmm1
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 320]
 movaps  xmm1, xmmword, ptr, [rbp, +, 336]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 mov     rcx, r12
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Line as geometric_algebra::RegressiveProduct<geometric_algebra::ppga3d::Line>>::regressive_product
 mov     ecx, eax
 call    geometric_algebra::ppga3d::<impl core::convert::From<geometric_algebra::ppga3d::Scalar> for f32>::from
 movaps  xmm8, xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 128]
 movaps  xmm1, xmmword, ptr, [rbp, +, 144]
 movaps  xmmword, ptr, [rbp, +, 544], xmm1
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 288]
 movaps  xmm1, xmmword, ptr, [rbp, +, 304]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 mov     rcx, r12
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Line as geometric_algebra::RegressiveProduct<geometric_algebra::ppga3d::Line>>::regressive_product
 mov     ecx, eax
 call    geometric_algebra::ppga3d::<impl core::convert::From<geometric_algebra::ppga3d::Scalar> for f32>::from
 addss   xmm8, xmm0
 movss   xmm0, dword, ptr, [r15, +, 60]
 divss   xmm0, xmm9
 addss   xmm0, xmm8
 divss   xmm7, xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 160]
 movaps  xmm1, xmmword, ptr, [rbp, +, 176]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 movaps  xmm2, xmm7
 xorps   xmm2, xmmword, ptr, [rip, +, __xmm@80000000800000008000000080000000]
 lea     rcx, [rbp, +, 416]
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Line as geometric_algebra::Scale>::scale
 movaps  xmm0, xmmword, ptr, [rbp, +, 128]
 movaps  xmm1, xmmword, ptr, [rbp, +, 144]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 384]
 mov     rdx, r13
 movaps  xmm2, xmm7
 call    <geometric_algebra::ppga3d::Line as geometric_algebra::Scale>::scale
 movaps  xmm0, xmmword, ptr, [rbp, +, 416]
 movaps  xmm1, xmmword, ptr, [rbp, +, 432]
 movaps  xmmword, ptr, [rbp, +, 544], xmm1
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 96]
 movaps  xmm1, xmmword, ptr, [rbp, +, 112]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 464]
 mov     rdx, r13
 mov     r8, r12
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::GeometricProduct<geometric_algebra::ppga3d::Line>>::geometric_product
 mov     rcx, r13
 lea     rdx, [rbp, +, 464]
 movaps  xmm2, xmm6
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Scale>::scale
 mov     rcx, rsi
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Motor as core::ops::arith::SubAssign>::sub_assign
 movaps  xmm0, xmmword, ptr, [rbp, +, 384]
 movaps  xmm1, xmmword, ptr, [rbp, +, 400]
 movaps  xmmword, ptr, [rbp, +, 544], xmm1
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 movaps  xmm0, xmmword, ptr, [rbp, +, 352]
 movaps  xmm1, xmmword, ptr, [rbp, +, 368]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 lea     rcx, [rbp, +, 464]
 mov     rdx, r13
 mov     r8, r12
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::GeometricProduct<geometric_algebra::ppga3d::Line>>::geometric_product
 mov     rcx, r13
 lea     rdx, [rbp, +, 464]
 movaps  xmm2, xmm6
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Scale>::scale
 add     rbx, rdi
 mov     rcx, rbx
 mov     rdx, r13
 call    <geometric_algebra::ppga3d::Motor as core::ops::arith::SubAssign>::sub_assign
 movaps  xmm0, xmmword, ptr, [rsi]
 movaps  xmm1, xmmword, ptr, [rsi, +, 16]
 movaps  xmmword, ptr, [rbp, +, 544], xmm1
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 movaps  xmm0, xmmword, ptr, [rsi]
 movaps  xmm1, xmmword, ptr, [rsi, +, 16]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 mov     rcx, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Magnitude>::magnitude
 mov     rcx, r13
 mov     rdx, r12
 mov     r8d, eax
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::GeometricQuotient<geometric_algebra::ppga3d::Scalar>>::geometric_quotient
 movaps  xmm0, xmmword, ptr, [rbp, +, 560]
 movaps  xmm1, xmmword, ptr, [rbp, +, 576]
 movaps  xmmword, ptr, [rsi, +, 16], xmm1
 movaps  xmmword, ptr, [rsi], xmm0
 movaps  xmm0, xmmword, ptr, [rbx]
 movaps  xmm1, xmmword, ptr, [rbx, +, 16]
 movaps  xmmword, ptr, [rbp, +, 544], xmm1
 movaps  xmmword, ptr, [rbp, +, 528], xmm0
 movaps  xmm0, xmmword, ptr, [rbx]
 movaps  xmm1, xmmword, ptr, [rbx, +, 16]
 movaps  xmmword, ptr, [rbp, +, 576], xmm1
 movaps  xmmword, ptr, [rbp, +, 560], xmm0
 mov     rcx, r13
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::Magnitude>::magnitude
 mov     rcx, r13
 mov     rdx, r12
 mov     r8d, eax
 call    <geometric_algebra::ppga3d::Motor as geometric_algebra::GeometricQuotient<geometric_algebra::ppga3d::Scalar>>::geometric_quotient
 movaps  xmm0, xmmword, ptr, [rbp, +, 560]
 movaps  xmm1, xmmword, ptr, [rbp, +, 576]
 movaps  xmmword, ptr, [rbx, +, 16], xmm1
 movaps  xmmword, ptr, [rbx], xmm0
 jmp     .LBB202_93
.LBB202_45:
 mov     rcx, qword, ptr, [rbp, +, 504]
 mov     rdx, qword, ptr, [rbp, +, 456]
 movaps  xmm6, xmmword, ptr, [rbp, +, 608]
 movaps  xmm7, xmmword, ptr, [rbp, +, 624]
 movaps  xmm8, xmmword, ptr, [rbp, +, 640]
 movaps  xmm9, xmmword, ptr, [rbp, +, 656]
 movaps  xmm10, xmmword, ptr, [rbp, +, 672]
 movaps  xmm11, xmmword, ptr, [rbp, +, 688]
 movaps  xmm12, xmmword, ptr, [rbp, +, 704]
 movaps  xmm13, xmmword, ptr, [rbp, +, 720]
 movaps  xmm14, xmmword, ptr, [rbp, +, 736]
 movaps  xmm15, xmmword, ptr, [rbp, +, 752]
 add     rsp, 904
 pop     rbx
 pop     rdi
 pop     rsi
 pop     r12
 pop     r13
 pop     r14
 pop     r15
 pop     rbp
 jmp     _ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$puffin..ProfilerScope$GT$$GT$17hd4ceb1b4ba1a6a7eE
.LBB202_52:
 lea     r8, [rip, +, __unnamed_125]
 mov     rcx, rsi
 call    core::panicking::panic_bounds_check
 jmp     .LBB202_53
.LBB202_51:
 lea     r8, [rip, +, __unnamed_126]
 mov     rcx, rbx
 call    core::panicking::panic_bounds_check
.LBB202_53:
 ud2
.LBB202_96:
 lea     rax, [rip, +, __unnamed_23]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_24]
 lea     r9, [rip, +, __unnamed_25]
 lea     r8, [rbp, +, 560]
 mov     edx, 16
 call    core::result::unwrap_failed
 ud2
.LBB202_95:
 lea     rax, [rip, +, __unnamed_121]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_101]
 mov     edx, 24
 mov     r9d, 24
 call    core::str::slice_error_fail
 ud2
.LBB202_94:
 lea     rax, [rip, +, __unnamed_123]
 mov     qword, ptr, [rsp, +, 32], rax
 lea     rcx, [rip, +, __unnamed_124]
 mov     edx, 60
 mov     r9d, 60
 call    core::str::slice_error_fail
 ud2
