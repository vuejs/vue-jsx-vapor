## [2.4.8](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.7...v2.4.8) (2025-07-05)


### Bug Fixes

* **compiler:** use parseExpression instead of walkIdentifiers ([13fa2f4](https://github.com/vuejs/vue-jsx-vapor/commit/13fa2f40e30ec3bcae2d2f438f35307e80f366de))



## [2.4.7](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.6...v2.4.7) (2025-07-04)


### Features

* support functional component in interop mode ([26afc8c](https://github.com/vuejs/vue-jsx-vapor/commit/26afc8ccf3887066feffefadd01782678f55a6a9))



## [2.4.6](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.5...v2.4.6) (2025-06-27)


### Bug Fixes

* **compiler-vapor:** properly locate last if node ([9633b9c](https://github.com/vuejs/vue-jsx-vapor/commit/9633b9cec201be95275a5f5328490dc3011bdcf7))
* **compiler:** correct execution order of operations ([485e709](https://github.com/vuejs/vue-jsx-vapor/commit/485e709e33a62ae095006ffbef7c830083ddcb85))
* **compiler:** prevent v-for components from being single root ([9c10b3b](https://github.com/vuejs/vue-jsx-vapor/commit/9c10b3b9018b2512efbbac243a2e16562ff7a32b))


### Features

* add sourceMap option ([9566477](https://github.com/vuejs/vue-jsx-vapor/commit/95664775d5750dc641bc60dfb4a52ac1832c4158))
* **vue-jsx-vapor:** add class and style for IntrinsicAttributes ([997045f](https://github.com/vuejs/vue-jsx-vapor/commit/997045feb0b77d05cec87704673d2736065c1e74))



## [2.4.5](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.4...v2.4.5) (2025-06-12)


### Features

* **eslint:** enhance offset calculation for defineStyle formatting ([7d53c74](https://github.com/vuejs/vue-jsx-vapor/commit/7d53c74642078ccadb0534322dcdb6a273503e1b))
* **macros:** improve handling of required props in defineComponent ([1889468](https://github.com/vuejs/vue-jsx-vapor/commit/1889468f68c70814e5c029b928aa0894e8a28886))



## [2.4.4](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.3...v2.4.4) (2025-06-03)


### Features

* **vue-jsx-vapor:** expose jsx-runtime/dom ([2802d74](https://github.com/vuejs/vue-jsx-vapor/commit/2802d7483965e46631b440f3d9b34f00e1eb3e10))



## [2.4.3](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.2...v2.4.3) (2025-06-01)


### Bug Fixes

* **compiler:** use createNodes instead of setNodes for v-slot ([1c5cf09](https://github.com/vuejs/vue-jsx-vapor/commit/1c5cf090c9edb9dde36309054ced58e81a58d711))



## [2.4.2](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.1...v2.4.2) (2025-05-29)


### Bug Fixes

* **comiler:** unwrap type for expressions ([a5c0c85](https://github.com/vuejs/vue-jsx-vapor/commit/a5c0c8521a0b4af0fba485214ff365b42d1f513b))
* **vue-jsx-vapor:** remove pauseTracking ([78fcc0a](https://github.com/vuejs/vue-jsx-vapor/commit/78fcc0a0b06e83539ca8baa4d2821b13d423b4b6))


### Features

* **vue-jsx-vapor:** support array expression ([bccb5ef](https://github.com/vuejs/vue-jsx-vapor/commit/bccb5ef9e7d3dd2115951c2b459694c06bd9be38))



## [2.4.1](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.0...v2.4.1) (2025-05-26)


### Bug Fixes

* **vue-jsx-vapor:** expose correct jsx-runtime type ([bdd3613](https://github.com/vuejs/vue-jsx-vapor/commit/bdd36133dc4e49efc4d1af916058ee655ddb8b83))


### Features

* **macros:** use vue-jsx-vapor/runtime to support browser environments ([20be1a0](https://github.com/vuejs/vue-jsx-vapor/commit/20be1a0ce7385ee4d1821444470f106c1e585039))



# [2.4.0](https://github.com/vuejs/vue-jsx-vapor/compare/v2.3.6...v2.4.0) (2025-05-26)


### Bug Fixes

* **babel:** add typescript plugin for parse ([2d0cd50](https://github.com/vuejs/vue-jsx-vapor/commit/2d0cd50245ac6c6add853b7c69c6a86758a0b9c4))
* **vue-jsx-vapor/volar:** correct passing macros's options ([21d8b0a](https://github.com/vuejs/vue-jsx-vapor/commit/21d8b0a390db96ad9f8a1f200715579d3dc97609))


### Features

* **macros/define-style:** support navigation for css-modules ([710127d](https://github.com/vuejs/vue-jsx-vapor/commit/710127d020fc10d2bb4a85c3cc83986fa2294440))
* **macros:** add JSX support for auto generate props ([4cfcc0d](https://github.com/vuejs/vue-jsx-vapor/commit/4cfcc0ded95f03281cf166c5ae0fdc309003e0d5))
* **macros:** props allowed to be overridden for defineComponent ([2f5ce4a](https://github.com/vuejs/vue-jsx-vapor/commit/2f5ce4ad78677c755764b9a199d70665c366baab))
* **vue-jsx-vapor:** add pauseTracking and pauseTracking for createNodes ([617eb2f](https://github.com/vuejs/vue-jsx-vapor/commit/617eb2f0c0ebb6155b11e4a1bf1b0e627f447e75))
* **vue-jsx-vapor:** introduce useProps and useFullProps ([69041a7](https://github.com/vuejs/vue-jsx-vapor/commit/69041a748f6908c9a1e1a2750107defd6a3308fb))
* **vue-jsx-vapor:** support custom HTMLAttributes for jsx-runtime ([22fb370](https://github.com/vuejs/vue-jsx-vapor/commit/22fb370ab8468268c1f7934326943c61d92f342d))



## [2.3.6](https://github.com/vuejs/vue-jsx-vapor/compare/v2.3.5...v2.3.6) (2025-05-12)


### Features

* **vue-jsx-vapor:** support vaporComponent for createNodes ([403a800](https://github.com/vuejs/vue-jsx-vapor/commit/403a800e60166bd225ebbfb9b60d4c3ee6f68dbb))



## [2.3.5](https://github.com/vuejs/vue-jsx-vapor/compare/v2.3.4...v2.3.5) (2025-05-05)


### Features

* **macros:** support :slotted for defineStyle ([fb10489](https://github.com/vuejs/vue-jsx-vapor/commit/fb10489f0674b0756ef8bbc7250b6ee545d7dc14))



## [2.3.4](https://github.com/vuejs/vue-jsx-vapor/compare/v2.3.3...v2.3.4) (2025-05-02)


### Bug Fixes

* **eslint/jsx-sort-props:** correct sort for reservedProps ([7ddd014](https://github.com/vuejs/vue-jsx-vapor/commit/7ddd01492bcb4d451683a71b8868538aa1ae1e12))
* **macros:** make alias option optional ([d459bd1](https://github.com/vuejs/vue-jsx-vapor/commit/d459bd1baf2f7fccd7830812aae30b7a680b701f))



## [2.3.3](https://github.com/vuejs/vue-jsx-vapor/compare/v2.3.2...v2.3.3) (2025-04-28)


### Features

* **compiler:** defaults prop.value to true when it is null ([e653e77](https://github.com/vuejs/vue-jsx-vapor/commit/e653e77f34202a4874eb26b82720775b0411cbfb))
* **macros:** automatically infer type from default value ([d7b31d6](https://github.com/vuejs/vue-jsx-vapor/commit/d7b31d67b70277141d427b036f4f09917c829238))
* **macros:** automatically infer type from default value for defineModel ([47c139e](https://github.com/vuejs/vue-jsx-vapor/commit/47c139e93d6cb6ee5f12e15de485ec2a2138c20b))



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



