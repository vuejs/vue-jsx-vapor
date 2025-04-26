## [2.3.2](https://github.com/vuejs/vue-jsx-vapor/compare/v2.3.1...v2.3.2) (2025-04-26)


### Bug Fixes

* **macros/volar:** add semicolon for defineComponent ([1289cb3](https://github.com/vuejs/vue-jsx-vapor/commit/1289cb3c13d6414fba6b2804109a647a9627a2c2))



## [2.3.1](https://github.com/vuejs/vue-jsx-vapor/compare/v2.3.0...v2.3.1) (2025-04-25)



# [2.3.0](https://github.com/vuejs/vue-jsx-vapor/compare/v2.2.0...v2.3.0) (2025-04-10)


### Features

* introduce eslint ([f241afa](https://github.com/vuejs/vue-jsx-vapor/commit/f241afa302d026dc0fa6e3d76ef2a26bfac9b37d))
* **macros/defineComponent:** support auto return functional component for SSR ([c0b310f](https://github.com/vuejs/vue-jsx-vapor/commit/c0b310f36453015201ed6c8e77f8b720310d8fda))



# [2.2.0](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.8...v2.2.0) (2025-04-05)


### Features

* **vue-jsx-vapor:** support SSR ([1ae4fe9](https://github.com/vuejs/vue-jsx-vapor/commit/1ae4fe9ceb66a280b689675c880a7870dac13160))



## [2.1.8](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.7...v2.1.8) (2025-03-28)


### Bug Fixes

* **vue-jsx-vapor:** prevent register HMR in production environment ([cab7de4](https://github.com/vuejs/vue-jsx-vapor/commit/cab7de4f3a9582ab8cb2a4d2d563563d7d8d053f))



## [2.1.7](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.6...v2.1.7) (2025-03-25)


### Bug Fixes

* **macros/volar:** use __MACROS_ctx to infer type ([0a5b315](https://github.com/vuejs/vue-jsx-vapor/commit/0a5b31559a909dc13197ec8d538f91bbfff5238c))


### Features

* **vue-jsx-vapor:** support hmr ([c1091da](https://github.com/vuejs/vue-jsx-vapor/commit/c1091dab41240b4b4d89d11d90eea284bc9bd414))
* **vue-jsx-vapor:** support hmr for functional components ([faed7fa](https://github.com/vuejs/vue-jsx-vapor/commit/faed7fa4b72e3a52bd28e03d945d90c44a450fa5))


### Performance Improvements

* **macors/volar:** optimize infer type performance ([bebec2b](https://github.com/vuejs/vue-jsx-vapor/commit/bebec2bea59f2f24a1772cb2a83964a7524d3acc))



## [2.1.6](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.5...v2.1.6) (2025-03-20)


### Bug Fixes

* **compiler:** use modelValueModifiers instead of modelModifiers ([563b2f0](https://github.com/vuejs/vue-jsx-vapor/commit/563b2f05437f72eb6cbf6a615c5def24ce95e3a6))
* **macros:** remove lib option ([9548729](https://github.com/vuejs/vue-jsx-vapor/commit/95487294b8e1953ad07ea29f22909cebdc626cf3))



## [2.1.5](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.4...v2.1.5) (2025-03-19)


### Features

* **compiler:** support v-text directive ([98a24d6](https://github.com/vuejs/vue-jsx-vapor/commit/98a24d62b13ecf0e6266939d417ae7ff4915426b))



## [2.1.4](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.3...v2.1.4) (2025-03-18)


### Features

* **vue-jsx-vapor:** use virtual code to support browser environments ([db1660e](https://github.com/vuejs/vue-jsx-vapor/commit/db1660edf5d109c55f2bd045fed5b9d08b436be2))



## [2.1.3](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.2...v2.1.3) (2025-03-18)


### Bug Fixes

* **compiler:** move next, child and nthChild before the setInsertionState ([d12a086](https://github.com/vuejs/vue-jsx-vapor/commit/d12a086739360bd3dafafd663191bc743208eb22))



## [2.1.2](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.1...v2.1.2) (2025-03-17)


### Bug Fixes

* **compiler:** remove log ([a1b9df0](https://github.com/vuejs/vue-jsx-vapor/commit/a1b9df07bc735ecaf45859af9477d9c9c05f653a))



## [2.1.1](https://github.com/vuejs/vue-jsx-vapor/compare/v2.1.0...v2.1.1) (2025-03-17)


### Bug Fixes

* **compiler:** missing modifiers ([32849a7](https://github.com/vuejs/vue-jsx-vapor/commit/32849a73c28f7da97dfc868f11d7a4aa0f45e4fe))
* **compiler:** remove empty modifiers ([dce2e83](https://github.com/vuejs/vue-jsx-vapor/commit/dce2e83d6892a47ef5a3a6132305bfed6e619269))
* **compiler:** use setInsertionState instead of insert ([2fae757](https://github.com/vuejs/vue-jsx-vapor/commit/2fae757c4323b93466cf233187b64968659c043c))


### Features

* **compiler:** support empty expression for event with modifiers ([187bf8c](https://github.com/vuejs/vue-jsx-vapor/commit/187bf8c1f0dbad4f726d48a10a9d487e46277f33))



# [2.1.0](https://github.com/vuejs/vue-jsx-vapor/compare/v2.0.1...v2.1.0) (2025-03-16)


### Bug Fixes

* lint ([9b7737d](https://github.com/vuejs/vue-jsx-vapor/commit/9b7737db48030f03a1816b162f3899e683ed8c27))


### Features

* **vue-jsx-vapor:** add jsxImportSource support ([b5e051d](https://github.com/vuejs/vue-jsx-vapor/commit/b5e051d14062b62f4e46e10b164997d443db08e2))



## [2.0.1](https://github.com/vuejs/vue-jsx-vapor/compare/v2.0.0...v2.0.1) (2025-03-12)


### Features

* **macros:** add semicolon prefix for defineExpose ([1f7171e](https://github.com/vuejs/vue-jsx-vapor/commit/1f7171e951543a71dc9a51250afcf448e6632e57))



# [2.0.0](https://github.com/vuejs/vue-jsx-vapor/compare/v1.7.0...v2.0.0) (2025-03-12)



# [1.7.0](https://github.com/vuejs/vue-jsx-vapor/compare/v1.6.0...v1.7.0) (2025-03-11)


### Bug Fixes

* lint ([a93b1e0](https://github.com/vuejs/vue-jsx-vapor/commit/a93b1e0f511a2f739d765b922383a09a64f19ae0))



# [1.6.0](https://github.com/vuejs/vue-jsx-vapor/compare/v1.5.0...v1.6.0) (2025-03-10)


### Features

* **volar:** auto infer type for useRef ([0e44f13](https://github.com/vuejs/vue-jsx-vapor/commit/0e44f131a0138f869c0ffd17e24b04a3e93c0c5f))



# [](https://github.com/vuejs/vue-jsx-vapor/compare/v1.0.9...v) (2025-03-09)


### Bug Fixes

* **babel:** compatibility with CommonJS ([00744bc](https://github.com/vuejs/vue-jsx-vapor/commit/00744bcd669830f99af864aa37bb6061ebde294b))
* **babel:** prevent slot errors by excluding conditional expressions ([c8b0171](https://github.com/vuejs/vue-jsx-vapor/commit/c8b01717545303100eee45710e3b508804ad8ea9))
* build error ([7bea325](https://github.com/vuejs/vue-jsx-vapor/commit/7bea325bbc8298e63fb80bb88288e1216b14e5f6))
* **compiler:** prevent handle comment node for v-slot ([5919124](https://github.com/vuejs/vue-jsx-vapor/commit/5919124be144fc0601cd831544e78a0caf736629))
* correct export path for api ([a6ec3a3](https://github.com/vuejs/vue-jsx-vapor/commit/a6ec3a33aa486220bd317f43b8b1f26afa62eefb))
* lint ([1289392](https://github.com/vuejs/vue-jsx-vapor/commit/12893921d9f004d31db8f99362ac71a29bebd68e))
* lint ([73c3534](https://github.com/vuejs/vue-jsx-vapor/commit/73c3534853f5580c5ccee8c5493478b6627fe848))
* lint ([1d08537](https://github.com/vuejs/vue-jsx-vapor/commit/1d08537f023ae6f4392a1fe1e7d349c1164f79cf))
* lint ([03a7140](https://github.com/vuejs/vue-jsx-vapor/commit/03a7140b6e89cd34fa6eb746323281e39ede74d4))
* lint ([b9edbb7](https://github.com/vuejs/vue-jsx-vapor/commit/b9edbb7f2c5e306a71555d13baf27b0246ba6257))
* lint ([1cda436](https://github.com/vuejs/vue-jsx-vapor/commit/1cda436714faf3f79405771a060f79b3aa4ea804))
* lint ([ba64de1](https://github.com/vuejs/vue-jsx-vapor/commit/ba64de187f672d9e17f21c4054825f84fa9cfc91))
* lint ([2abc350](https://github.com/vuejs/vue-jsx-vapor/commit/2abc35088596499343fbd49992dadda727b9fa4b))
* remove any ([abf0d0c](https://github.com/vuejs/vue-jsx-vapor/commit/abf0d0c6a1ea4991f1627a251163d8a3bc22ef87))
* typecheck ([ecbbbab](https://github.com/vuejs/vue-jsx-vapor/commit/ecbbbab33145e633561405847efd4bf7cf229c98))
* typecheck ([31787c2](https://github.com/vuejs/vue-jsx-vapor/commit/31787c2028c53300b5128910d0a8d51e0c41cb27))
* **unplugin:** correct type for raw.ts ([ba206e6](https://github.com/vuejs/vue-jsx-vapor/commit/ba206e61d66bd73d0fd3c00ef9061c711c592457))
* **unplugin:** typecheck ([ab8cbfe](https://github.com/vuejs/vue-jsx-vapor/commit/ab8cbfeba1cb4bd2a05f201908c5b2259e223d77))


### Features

* add interop mode ([f46592c](https://github.com/vuejs/vue-jsx-vapor/commit/f46592c2913d484511d2e067fb079bd3d7a68312))
* **babel:** support nested source maps ([3e69eba](https://github.com/vuejs/vue-jsx-vapor/commit/3e69eba92a2a5be9a21fc85b6433f1008850d851))
* **babel:** support source map ([17d7ea7](https://github.com/vuejs/vue-jsx-vapor/commit/17d7ea708c8a313e3811312c348a55b1abce7a6c))
* **compiler:** remove babel/parser ([ac494d1](https://github.com/vuejs/vue-jsx-vapor/commit/ac494d1ea62ca4f61bc8d0ceee9bc47e1f06e606))
* **compiler:** support native v-if directive ([18b77fc](https://github.com/vuejs/vue-jsx-vapor/commit/18b77fc49df3a500fafa67ebd69ada729bcb7ab1))
* **compiler:** support native v-slot directive ([9db436d](https://github.com/vuejs/vue-jsx-vapor/commit/9db436dd04e8e8214c3d179586a3b80f63a52777))
* **compiler:** support nested component slot ([a17e04e](https://github.com/vuejs/vue-jsx-vapor/commit/a17e04ebb5ecbe8d7b7fd37206ba1fc083351e03))
* **compiler:** support string source ([4293948](https://github.com/vuejs/vue-jsx-vapor/commit/42939489c6d89d5dce1e7d9c2640ec8c07f3ab44))
* **compiler:** support v-once directive ([16a6b49](https://github.com/vuejs/vue-jsx-vapor/commit/16a6b49ea976209adfb50abd84d24c5bd4641326))
* **compiler:** support v-text directive ([e1445ae](https://github.com/vuejs/vue-jsx-vapor/commit/e1445ae907837351fe64419173c1bd10b8e29600))
* introducing babel-plugin ([dd2e384](https://github.com/vuejs/vue-jsx-vapor/commit/dd2e3840030e3b35b682baa6ea4d84516f7de556))
* support AST node compilation ([06eb0a3](https://github.com/vuejs/vue-jsx-vapor/commit/06eb0a3a8e8247a1c9c93113a958dd1cdd1cf47b))
* **unplugin:** add filename option for source map ([10a6909](https://github.com/vuejs/vue-jsx-vapor/commit/10a69095e16f642993eee760d5b44f7d6f0b658f))
* **unplugin:** add volar plugin ([7bf1284](https://github.com/vuejs/vue-jsx-vapor/commit/7bf1284468483f3567aad6466d363ae2928d6a6c))
* **unplugin:** expose raw to support browser environments ([4dc2ffc](https://github.com/vuejs/vue-jsx-vapor/commit/4dc2ffcce47f888cfb6f9f66ef1b2863401b606d))


### Reverts

* **unplugin:** add effectScope for helper code ([dfb640b](https://github.com/vuejs/vue-jsx-vapor/commit/dfb640b30c59ee0103a84e73f40ba803a25050e9))
* vue/vapor ([ea9f738](https://github.com/vuejs/vue-jsx-vapor/commit/ea9f738f6ce1a580d14c0518df29d8cae1041434))



