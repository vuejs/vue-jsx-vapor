## [3.1.22](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.21...v3.1.22) (2026-02-13)


### Bug Fixes

* **compiler/vapor:** prevent generate template when fragment has ([997d963](https://github.com/vuejs/vue-jsx-vapor/commit/997d9630ee850915a9cc8c09aaa7e8d2104bd1ec))
* **compiler:** remove unused defineVaporComponent for SSR ([9639575](https://github.com/vuejs/vue-jsx-vapor/commit/963957534f49b3c1c3e1e6434bb5b1e33616395b))
* **hmr:** use reload instead of rerender for functional component ([d3f65fb](https://github.com/vuejs/vue-jsx-vapor/commit/d3f65fb4befca08f30c89e02e56f93f36684e2ef))
* **macros/volar:** unwrapRef for exposed ([60b6c63](https://github.com/vuejs/vue-jsx-vapor/commit/60b6c6396300f309450689805a9b79dfbf9c621c))
* **macros/volar:** use defineVaporComponent instead of functional component ([33bf5ce](https://github.com/vuejs/vue-jsx-vapor/commit/33bf5ce931ba2b7fc9cbd4982d98b9f4559a30a1))



## [3.1.21](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.20...v3.1.21) (2026-02-11)


### Bug Fixes

* **compiler/vapor:** always set index for if IR ([eef760d](https://github.com/vuejs/vue-jsx-vapor/commit/eef760de21af736c05c3e68348aeb7bd9c1a2f0e))
* **compiler/vapor:** skip static key ([505db7e](https://github.com/vuejs/vue-jsx-vapor/commit/505db7e3d2b1231e6f39e3f421bedbd343c95cd4))
* **compiler:** ensure JSX is compiled to Vapor in defineVaporComponent ([148746b](https://github.com/vuejs/vue-jsx-vapor/commit/148746b4f257e7248d72f1134613ecd30bcc0db1))
* **runtime:** set __proxyed for createProxyComponent directly ([e6de6aa](https://github.com/vuejs/vue-jsx-vapor/commit/e6de6aa4da01afe76b3fd1d0f6e431506ad7e3ae))



## [3.1.20](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.19...v3.1.20) (2026-02-03)


### Bug Fixes

* **compiler/vapor:** handle invalid table nesting with dynamic child ([712ec26](https://github.com/vuejs/vue-jsx-vapor/commit/712ec26e030218cbdc680a60788deebb8b3c53dc))
* **compiler:** use createNodes instead of setNodes for fragment ([37be7f2](https://github.com/vuejs/vue-jsx-vapor/commit/37be7f247ae665144d1751d3765aaeed102377dd))
* use regex instead of string for filterId ([fac1483](https://github.com/vuejs/vue-jsx-vapor/commit/fac1483ae372eaa45b4ae8acfc6ae56e8b658a1f))


### Features

* **compiler/vapor:** support nested fragments ([38d2925](https://github.com/vuejs/vue-jsx-vapor/commit/38d29252a42f40f5ba70d06ef06e517ef0b022ab))



## [3.1.19](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.18...v3.1.19) (2026-02-01)


### Bug Fixes

* **compiler/vapor:** add key for transition if-branches ([f711dc6](https://github.com/vuejs/vue-jsx-vapor/commit/f711dc64a102cb43e5592d107800ed10b89b7170))
* **compiler/vapor:** do not escape quotes in root-level text nodes ([d843400](https://github.com/vuejs/vue-jsx-vapor/commit/d8434007b043b0b541d3e149a93c0b5244dceb47))
* **compiler/vapor:** enhance v-slot prop destructuring support ([5c7ef06](https://github.com/vuejs/vue-jsx-vapor/commit/5c7ef0652ca0aaf29489b531db2e4ce29a4fe2af))
* **compiler/vapor:** merge component v-model onUpdate handlers ([01ed414](https://github.com/vuejs/vue-jsx-vapor/commit/01ed414171ac4ca338e05c093c58c42ed2efd7f9))
* **compiler/vapor:** prevent event handler execution during emits lookup ([946fa2f](https://github.com/vuejs/vue-jsx-vapor/commit/946fa2fd5e5a2819e6365592d2ce6604b894ba43))
* **compiler/vapor:** support v-if and v-for on the same <template> ([33d97ef](https://github.com/vuejs/vue-jsx-vapor/commit/33d97ef9aa874ebce359b372f2d6346d0418e3fd))
* **compiler/vapor:** wrap handler values in functions for dynamic v-on ([514a200](https://github.com/vuejs/vue-jsx-vapor/commit/514a200e937fc040e21befcf8dd20513d8976ac6))
* use regex instead of string for filter.id ([f54d685](https://github.com/vuejs/vue-jsx-vapor/commit/f54d6852570b4406eb9ddf2bfa0e2ff31c5dbaca))



## [3.1.18](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.17...v3.1.18) (2026-01-29)


### Bug Fixes

* **compiler/vapor:** more accurate fallthrough attr support ([4c571ef](https://github.com/vuejs/vue-jsx-vapor/commit/4c571eff62574629c493813e73040013fd1a5349))
* **compiler/vapor:** only apply v-on key modifiers to keyboard events ([1bb06d8](https://github.com/vuejs/vue-jsx-vapor/commit/1bb06d8d1dc954f538200a664a63b40f666ba83e))
* **compiler/vapor:** prevent nested components from inheriting parent ([f5539ec](https://github.com/vuejs/vue-jsx-vapor/commit/f5539ec85bab110fc90073f8de8848a71c063cd1))
* **volar/jsx-element:** refine type inference ([dc21389](https://github.com/vuejs/vue-jsx-vapor/commit/dc21389ecd0dedb29e7e37c0983d8cc238c5d44f))


### Features

* **compiler/vapor:** support merging multiple event handlers on ([4471831](https://github.com/vuejs/vue-jsx-vapor/commit/447183183fbb2ab523928974a1a5a868c7672f63))
* **macros/volar:** support auto infer slot types for defineComponent ([1f7f474](https://github.com/vuejs/vue-jsx-vapor/commit/1f7f4741f00db342c13e772b5f82b1c12d643107))



## [3.1.17](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.16...v3.1.17) (2026-01-28)


### Bug Fixes

* **compiler/vdom:** merge v-model and onUpdate handlers ([501dfb6](https://github.com/vuejs/vue-jsx-vapor/commit/501dfb64a2cc49c963cb83b7109618b1cf564965)), closes [#43](https://github.com/vuejs/vue-jsx-vapor/issues/43)
* **macros:** use proxy slots to keep reactivity ([e834f13](https://github.com/vuejs/vue-jsx-vapor/commit/e834f13af78a7b789db2027061536f61ad7a37c1))
* **runtime:** remove __NO_SIDE_EFFECTS__ flag ([41d6e8d](https://github.com/vuejs/vue-jsx-vapor/commit/41d6e8d9cc686b87da3993fcbc49807fc5dc8823))


### Features

* **macros/volar:** support auto infer slot type ([c1325cd](https://github.com/vuejs/vue-jsx-vapor/commit/c1325cdd2c87f5f269b0c29ee2588f277b82d50a))



## [3.1.16](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.15...v3.1.16) (2026-01-27)


### Bug Fixes

* **compiler/vdom:** prevent generate redundant normalizeVNode when with ([f83bf65](https://github.com/vuejs/vue-jsx-vapor/commit/f83bf65da63bc45a626115ad257149bb732768d9)), closes [#42](https://github.com/vuejs/vue-jsx-vapor/issues/42)
* **compiler:** prevent mark _temp as static for LogicalExpression ([f5ef8f5](https://github.com/vuejs/vue-jsx-vapor/commit/f5ef8f5c30c3518df49e9bbb4ba534539ea0ccc4)), closes [#41](https://github.com/vuejs/vue-jsx-vapor/issues/41)


### Features

* **compiler/vapor:** add withVaporCtx helper to manage currentInstance ([47929f9](https://github.com/vuejs/vue-jsx-vapor/commit/47929f9ec95f3fcd8782b85b1544f190c44376e3))
* **compiler/vapor:** support defineVaporCustomElement ([420c903](https://github.com/vuejs/vue-jsx-vapor/commit/420c9036a6d0470b9391b039d403c0c1e318a933))
* **compiler/vapor:** support slot ([1e0767b](https://github.com/vuejs/vue-jsx-vapor/commit/1e0767b84ff5e69f930003e3d921e9af0d523deb))
* **compiler/vdom:** support slot ([9b3d7e8](https://github.com/vuejs/vue-jsx-vapor/commit/9b3d7e85a9c53658815b15076b6bf7a7613284bc))
* support custom-element ([82dd87f](https://github.com/vuejs/vue-jsx-vapor/commit/82dd87f195121c636728b7b783a1c2831596980a))



## [3.1.15](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.14...v3.1.15) (2026-01-24)


### Bug Fixes

* **runtime:** expose SSR ([9c8ff33](https://github.com/vuejs/vue-jsx-vapor/commit/9c8ff33412738c165e72f45bf69d81a15e7110e7))


### Features

* **compiler/vapor:** support svg and MathML ([98f47f4](https://github.com/vuejs/vue-jsx-vapor/commit/98f47f4ac93d8fec5439d1d17d74f2deaac39e10))



## [3.1.14](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.13...v3.1.14) (2026-01-23)


### Bug Fixes

* **compiler/vdom:** prevent mark condition expression as static ([a6c0cb5](https://github.com/vuejs/vue-jsx-vapor/commit/a6c0cb53481089f53b421134cb8d6bff427d83db))


### Features

* **compiler:** support custom runtimeModuleName ([fa772d1](https://github.com/vuejs/vue-jsx-vapor/commit/fa772d1fa2aee6fd39ee2bab23255cf02847a2f6))
* support bun plugin ([00aa94b](https://github.com/vuejs/vue-jsx-vapor/commit/00aa94b69c19611926d0b669d79be6d8af6bc075))



## [3.1.13](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.12...v3.1.13) (2026-01-22)


### Bug Fixes

* **compiler/vapor:** remove empty_text child ([bf87609](https://github.com/vuejs/vue-jsx-vapor/commit/bf8760972b3cf550a27560b6fbea741927835d46))
* **compiler:** preserve non-breaking spaces in JSX ([c104e6a](https://github.com/vuejs/vue-jsx-vapor/commit/c104e6afe44f10f9539201cd911d35bc5fa9b00a))
* **compiler:** regist ssr moduels for defineVaporSSRComponent ([f4d25a7](https://github.com/vuejs/vue-jsx-vapor/commit/f4d25a79ce4c771e41bfffb9b55814634e123b41))


### Features

* **compiler/vapor:** hydration ([e3722c6](https://github.com/vuejs/vue-jsx-vapor/commit/e3722c675ad26046ede23859dfb555e49990db9b))
* **compiler/vapor:** v-html and v-text work with component ([24d8cd8](https://github.com/vuejs/vue-jsx-vapor/commit/24d8cd8595519739d6c18d173921928df74422e7))
* support hydration ([3f235d9](https://github.com/vuejs/vue-jsx-vapor/commit/3f235d9d3d5cc5caaeccc907230539e749d2ff1e))



## [3.1.12](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.10...v3.1.12) (2026-01-15)


### Bug Fixes

* **compiler:** escape HTML chars ([9a9ae27](https://github.com/vuejs/vue-jsx-vapor/commit/9a9ae272f3746d97edcda58843b5b1d765ba58c2))
* **compiler:** prevent process JSXEmptyExpression for props ([e67b463](https://github.com/vuejs/vue-jsx-vapor/commit/e67b463a585da05546adfc890575763c09b55435))
* **compiler:** prevent process value prop as directive ([cc9c31a](https://github.com/vuejs/vue-jsx-vapor/commit/cc9c31a884879740068eb8b6a0413c43f95085a2))
* **compiler:** wrapping resolveDirective for custom directive ([e1a37b8](https://github.com/vuejs/vue-jsx-vapor/commit/e1a37b858cfd4095d8362b2e2677c467830bcdad))


### Features

* **compiler:** support Transition and TransitionGroup ([13bd49b](https://github.com/vuejs/vue-jsx-vapor/commit/13bd49b95be437711252fed6ea8a185a6a2617f9))
* **hmr:** support defineComponentName option ([1664b82](https://github.com/vuejs/vue-jsx-vapor/commit/1664b82289e353c32c767f0d683ed1103c003777)), closes [#35](https://github.com/vuejs/vue-jsx-vapor/issues/35)
* **vapor:** support key ([89b502d](https://github.com/vuejs/vue-jsx-vapor/commit/89b502dbc3e5c02e0b1a87cb0c5f6e898d087f29))


### Performance Improvements

* **compiler:** remove IIFE that only returns an expression ([ac72506](https://github.com/vuejs/vue-jsx-vapor/commit/ac725066c6ab4bfb4f13f485da0df86367d8cea7))
* **compiler:** use oxc_ast_visit instead of oxc_traverse ([32b789e](https://github.com/vuejs/vue-jsx-vapor/commit/32b789eb15fbc3560598e0a2070b7227e4817f98))
* **macros:** add code filter hooks ([fe8d82f](https://github.com/vuejs/vue-jsx-vapor/commit/fe8d82f83b90e9774f10298b345471cac405d8c9))
* **vapor:** use Direcives struct instead of find_prop ([ede1ab3](https://github.com/vuejs/vue-jsx-vapor/commit/ede1ab3ce2bc40bf55de3dfd25647f6937f72229))



## [3.1.11](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.10...v3.1.11) (2026-01-11)


### Features

* **hmr:** support defineComponentName option ([1664b82](https://github.com/vuejs/vue-jsx-vapor/commit/1664b82289e353c32c767f0d683ed1103c003777)), closes [#35](https://github.com/vuejs/vue-jsx-vapor/issues/35)


### Performance Improvements

* **compiler:** remove IIFE that only returns an expression ([ac72506](https://github.com/vuejs/vue-jsx-vapor/commit/ac725066c6ab4bfb4f13f485da0df86367d8cea7))
* **compiler:** use oxc_ast_visit instead of oxc_traverse ([32b789e](https://github.com/vuejs/vue-jsx-vapor/commit/32b789eb15fbc3560598e0a2070b7227e4817f98))
* **macros:** add code filter hooks ([fe8d82f](https://github.com/vuejs/vue-jsx-vapor/commit/fe8d82f83b90e9774f10298b345471cac405d8c9))
* **vapor:** use Direcives struct instead of find_prop ([ede1ab3](https://github.com/vuejs/vue-jsx-vapor/commit/ede1ab3ce2bc40bf55de3dfd25647f6937f72229))



## [3.1.10](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.9...v3.1.10) (2026-01-06)


### Features

* **vue-jsx-vapor:** use virtual helper modules to prevent deps on ([654da93](https://github.com/vuejs/vue-jsx-vapor/commit/654da93778c30727f313c691f76ad66965c1d37e))



## [3.1.9](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.8...v3.1.9) (2026-01-02)



## [3.1.8](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.7...v3.1.8) (2026-01-02)


### Bug Fixes

* **compiler/dom:** slots in v-for should be dynamic ([d6fd49e](https://github.com/vuejs/vue-jsx-vapor/commit/d6fd49eb7b4105d1f581b253f785939f8840aedf))



## [3.1.7](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.6...v3.1.7) (2025-12-31)



## [3.1.6](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.5...v3.1.6) (2025-12-31)


### Bug Fixes

* **compiler/vdom:** prevent cache include slot props's events for ([0ce3a55](https://github.com/vuejs/vue-jsx-vapor/commit/0ce3a55f3ca14b689c02ceded9d4c056a6e60068)), closes [#30](https://github.com/vuejs/vue-jsx-vapor/issues/30)
* **compiler:** add missing _temp variable ([2b4b691](https://github.com/vuejs/vue-jsx-vapor/commit/2b4b6916594a9d1b1e0be7a993071a56e443f8c7)), closes [#31](https://github.com/vuejs/vue-jsx-vapor/issues/31)
* **compiler:** remove unused withCtx helper for Fragment ([afdec4a](https://github.com/vuejs/vue-jsx-vapor/commit/afdec4a4000a40e0c1493c12085ab19cb820ebdb))



## [3.1.5](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.4...v3.1.5) (2025-12-30)


### Bug Fixes

* **compiler:** use cache for LogicalExpression ([f3e14b3](https://github.com/vuejs/vue-jsx-vapor/commit/f3e14b32b6f56f3c1cae7f49f60969e8e3bf3316))
* **runtime:** use Text instead of Comment to support undefined ref ([e0e5a4d](https://github.com/vuejs/vue-jsx-vapor/commit/e0e5a4d5cde58f76df36131a6c88f9057b05ae4b))


### Features

* **compiler:** convert ObjectExpression and FunctionExpression to ([38e6057](https://github.com/vuejs/vue-jsx-vapor/commit/38e60575606ddf65d152521a02c8c2277e3b4246)), closes [#29](https://github.com/vuejs/vue-jsx-vapor/issues/29)
* **compiler:** support LogicalExpression with coalesce ([0f48e4c](https://github.com/vuejs/vue-jsx-vapor/commit/0f48e4c625905c0ebc896b254e7ebf66481d3183))



## [3.1.4](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.3...v3.1.4) (2025-12-29)


### Bug Fixes

* **runtime:** prevent render undefined ([7b8c058](https://github.com/vuejs/vue-jsx-vapor/commit/7b8c0582a5e481cc542cb8fc7f75ce35d5bdf3e6)), closes [#27](https://github.com/vuejs/vue-jsx-vapor/issues/27)


### Features

* **hmr:** support rerender render function ([4c394e5](https://github.com/vuejs/vue-jsx-vapor/commit/4c394e5fd6751192fc46f461387d6c5b9703c3a5)), closes [#28](https://github.com/vuejs/vue-jsx-vapor/issues/28)
* **vue-jsx-vapor:** add compiler options ([1bb3932](https://github.com/vuejs/vue-jsx-vapor/commit/1bb393268f318303155a4e418d2372ab84e64d33))


### Performance Improvements

* **compiler/vdom:** use take_in instead of clone_in for JSXExpression ([76c53bd](https://github.com/vuejs/vue-jsx-vapor/commit/76c53bd23d51a67b877960b49284f1b4f9e38dad))
* **compiler:** remove empty_text ([c796b83](https://github.com/vuejs/vue-jsx-vapor/commit/c796b837cd9a31af0afa87e190ba789fc8b1536d))



## [3.1.3](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.2...v3.1.3) (2025-12-26)


### Bug Fixes

* add index for createVNodeCache ([0c9af89](https://github.com/vuejs/vue-jsx-vapor/commit/0c9af89949516a1e018d85daccbafd99179d9b1e))


### Features

* **compiler/vdom:** check key for v-memo in v-for ([6f7f1a3](https://github.com/vuejs/vue-jsx-vapor/commit/6f7f1a3c5d5fbbdfde2988ffdf4dfe5e7ec133a5))
* **compiler/vdom:** support map expression ([ab16f06](https://github.com/vuejs/vue-jsx-vapor/commit/ab16f06e577bceced0031ac4eba176a71a42e4d5))


### Performance Improvements

* **compiler:** use createElementBlock instead of createBlock for v-for ([7a86c07](https://github.com/vuejs/vue-jsx-vapor/commit/7a86c072571ed84ace5f022369e76894c013433a))



## [3.1.2](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.1...v3.1.2) (2025-12-26)


### Bug Fixes

* **compiler:** prevent v-for error in browser ([1cf005b](https://github.com/vuejs/vue-jsx-vapor/commit/1cf005ba1fdeeb17a2b6e27e431a137e2e336113))



## [3.1.1](https://github.com/vuejs/vue-jsx-vapor/compare/v3.1.0...v3.1.1) (2025-12-25)


### Bug Fixes

* **compiler/vdom:** prevent JSXText.raw unwrap error in browser ([85699ae](https://github.com/vuejs/vue-jsx-vapor/commit/85699ae0f3f87587ab9989c4b3b4d6e8635c1166))



# [3.1.0](https://github.com/vuejs/vue-jsx-vapor/compare/v3.0.4...v3.1.0) (2025-12-25)


### Features

* **compiler:** support generating Virtual DOM ([#26](https://github.com/vuejs/vue-jsx-vapor/issues/26)) ([c1701db](https://github.com/vuejs/vue-jsx-vapor/commit/c1701dbd691c95db37cb83fd39bbd75aec8730e1))



## [3.0.4](https://github.com/vuejs/vue-jsx-vapor/compare/v3.0.3...v3.0.4) (2025-12-10)


### Features

* normalize setup-fn result ([8fa28ad](https://github.com/vuejs/vue-jsx-vapor/commit/8fa28add84e4d47dd68e1ebe9eac7fb2ec832d4a))



## [3.0.3](https://github.com/vuejs/vue-jsx-vapor/compare/v3.0.2...v3.0.3) (2025-12-10)


### Bug Fixes

* **runtime:** prevent proxy setup fn multiple ([84cd14b](https://github.com/vuejs/vue-jsx-vapor/commit/84cd14bd1419c2a3f8839866e51fdbd0704e8acb)), closes [#24](https://github.com/vuejs/vue-jsx-vapor/issues/24)



## [3.0.2](https://github.com/vuejs/vue-jsx-vapor/compare/v3.0.1...v3.0.2) (2025-12-09)


### Bug Fixes

* **macros:** sync localValue with prop changes for defineModel ([95d2d89](https://github.com/vuejs/vue-jsx-vapor/commit/95d2d89e8410fb246f2da98e53f72e37f4b0fdea))



## [3.0.1](https://github.com/vuejs/vue-jsx-vapor/compare/v3.0.0...v3.0.1) (2025-12-03)


### Bug Fixes

* **compiler-rs:** prevent compiler functional component to vapor when ([2e30262](https://github.com/vuejs/vue-jsx-vapor/commit/2e302628c301aa3706e25737cf6470e1b61becff))



# [3.0.0](https://github.com/vuejs/vue-jsx-vapor/compare/v2.7.4...v3.0.0) (2025-11-29)



## [2.7.4](https://github.com/vuejs/vue-jsx-vapor/compare/v2.7.3...v2.7.4) (2025-11-29)


### Bug Fixes

* **compiler-rs:** move context.slots to block.slots for nested slots ([717a6ac](https://github.com/vuejs/vue-jsx-vapor/commit/717a6ace4c078a2e134bc62ba01f496fee03f695))
* **vue-jsx-vapor:** move macros before compiler-rs ([7332859](https://github.com/vuejs/vue-jsx-vapor/commit/7332859fd69c05d4a80d50527465cfbc2dfb2d33))



## [2.7.3](https://github.com/vuejs/vue-jsx-vapor/compare/v2.7.2...v2.7.3) (2025-11-28)



## [2.7.2](https://github.com/vuejs/vue-jsx-vapor/compare/v2.7.1...v2.7.2) (2025-11-28)



## [2.7.1](https://github.com/vuejs/vue-jsx-vapor/compare/v2.7.0...v2.7.1) (2025-11-28)



# [2.7.0](https://github.com/vuejs/vue-jsx-vapor/compare/v2.6.8...v2.7.0) (2025-11-28)


### Bug Fixes

* **compiler:** prevent transforming v-slot with non-ObjectExpression ([73be347](https://github.com/vuejs/vue-jsx-vapor/commit/73be34759e7e9b05b84a6eadcb3f6faf080b827d))



## [2.6.8](https://github.com/vuejs/vue-jsx-vapor/compare/v2.6.7...v2.6.8) (2025-10-22)


### Bug Fixes

* **macros:** return VNode for generic defineComponent ([3bd71f8](https://github.com/vuejs/vue-jsx-vapor/commit/3bd71f8d5e69c7e6353fc570ffeee2ed6e5a6b8a)), closes [#21](https://github.com/vuejs/vue-jsx-vapor/issues/21)



## [2.6.7](https://github.com/vuejs/vue-jsx-vapor/compare/v2.6.6...v2.6.7) (2025-09-30)


### Bug Fixes

* **compiler:** escape html for safer template output ([d02e238](https://github.com/vuejs/vue-jsx-vapor/commit/d02e238ca77788d0b616ab60365d55609ca3aa5c))
* **compiler:** treat template v-for with single component child as component ([f657781](https://github.com/vuejs/vue-jsx-vapor/commit/f657781117713914adce7156d272b80132afb382))



## [2.6.6](https://github.com/vuejs/vue-jsx-vapor/compare/v2.6.4...v2.6.6) (2025-09-23)


### Bug Fixes

* **macros:** add placeholder after removed params ([e7b81c1](https://github.com/vuejs/vue-jsx-vapor/commit/e7b81c12d0c5a62225bc79921f4571f001b75daa))
* **macros:** prevent handle dts files ([a34b23c](https://github.com/vuejs/vue-jsx-vapor/commit/a34b23c589e28a151236715255ecda7f2268857c))
* **runtime:** correct expose types for VaporComponentInstance ([ebe60a6](https://github.com/vuejs/vue-jsx-vapor/commit/ebe60a690a4a45cc1d7dc017e7a750dc29c604c1))
* **runtime:** handle undefined default slot for Fragment ([8a349d1](https://github.com/vuejs/vue-jsx-vapor/commit/8a349d173b7969e16b41a96c257a014b34c57f11))


### Features

* **macros:** add "/" prefix for helper-id ([341d18c](https://github.com/vuejs/vue-jsx-vapor/commit/341d18c35c442ffa5e962dd088a99f7b63705bdd))
* **macros:** support generic components ([ff75eb7](https://github.com/vuejs/vue-jsx-vapor/commit/ff75eb794b3352734498fbb32e3a943beab9fb1c))



## [2.6.4](https://github.com/vuejs/vue-jsx-vapor/compare/v2.6.3...v2.6.4) (2025-09-06)


### Bug Fixes

* **volar/jsx-element:** return VaporInstance for functional component ([3b14249](https://github.com/vuejs/vue-jsx-vapor/commit/3b142496a05993f85d1f41a8824a4ab22affa0fd))



## [2.6.3](https://github.com/vuejs/vue-jsx-vapor/compare/v2.6.2...v2.6.3) (2025-09-06)


### Bug Fixes

* **compiler:** adjust children generation order for hydration ([128eb21](https://github.com/vuejs/vue-jsx-vapor/commit/128eb213879dc5279d36edc79f74d578c3b3d31c))


### Features

* **volar:** introduce jsx-element ([264c7be](https://github.com/vuejs/vue-jsx-vapor/commit/264c7be9cf98d11d70d3015f20bc86b31fe3aca6))



## [2.6.2](https://github.com/vuejs/vue-jsx-vapor/compare/v2.6.1...v2.6.2) (2025-08-08)


### Bug Fixes

* **babel:** import createComponentWithFallback from vue-jsx-vapor ([0c5b966](https://github.com/vuejs/vue-jsx-vapor/commit/0c5b96608e61a13fdf8f9d9e14df5bca8ffc7a1e))
* **runtime:** correct props type for h ([443945b](https://github.com/vuejs/vue-jsx-vapor/commit/443945bbc01446fcdd025385aa05880d59441052))
* **runtime:** normalizeNode slots for createComponentWithFallback ([2f8e6fc](https://github.com/vuejs/vue-jsx-vapor/commit/2f8e6fcfca11cce4b098eac15a083beebd37674d))


### Features

* **compiler:** add withFallback option to compile components to createComponentWithFallback ([0c2d470](https://github.com/vuejs/vue-jsx-vapor/commit/0c2d4703e88028867846b50cbcccc1763d982c82))
* **macros:** add verification feature for return keyword ([65c6a5e](https://github.com/vuejs/vue-jsx-vapor/commit/65c6a5ef30b3bc56790f598a89b05cff36cb0093))



## [2.6.1](https://github.com/vuejs/vue-jsx-vapor/compare/v2.6.0...v2.6.1) (2025-08-05)


### Bug Fixes

* **macros:** correct types for defineComponent and defineVaporComponent ([8a79bdb](https://github.com/vuejs/vue-jsx-vapor/commit/8a79bdbbbe5252f325c27ee22f69bfcd4e398bc9))



# [2.6.0](https://github.com/vuejs/vue-jsx-vapor/compare/v2.5.4-beta.1...v2.6.0) (2025-07-29)


### Bug Fixes

* **macros:** allow VariableStatement for defineModel ([9769f27](https://github.com/vuejs/vue-jsx-vapor/commit/9769f271d4de2734f0e0e14a8532e24bbefe1e21))
* **macros:** prevent return defineComponent for generic component ([dc64b7f](https://github.com/vuejs/vue-jsx-vapor/commit/dc64b7f172dfdef8e4dd5ed86d6f10bd776e58e9))
* **macros:** return DefineComponent type for defineComponent ([5803b5e](https://github.com/vuejs/vue-jsx-vapor/commit/5803b5ee29500b28350bb44883cf83d21122a5c8))


### Features

* introduce @vue-jsx-vapor/runtime ([27add9e](https://github.com/vuejs/vue-jsx-vapor/commit/27add9e4938c854a337598adae37ecfa0caa1ccb))
* **vue-jsx-vapor:** support functional props for h ([4fb5d61](https://github.com/vuejs/vue-jsx-vapor/commit/4fb5d616d4fd6f143c6482b22e1267211d3892f1))



## [2.5.4-beta.1](https://github.com/vuejs/vue-jsx-vapor/compare/v2.5.3...v2.5.4-beta.1) (2025-07-23)



## [2.5.3](https://github.com/vuejs/vue-jsx-vapor/compare/v2.5.2...v2.5.3) (2025-07-22)


### Features

* **compiler:** add fast-remove flag ([4e549e1](https://github.com/vuejs/vue-jsx-vapor/commit/4e549e193bd12826001a15a7c95103f0cd4f900e))
* **compiler:** support JSXComponent in JSXExpressionContainer ([c69e8ca](https://github.com/vuejs/vue-jsx-vapor/commit/c69e8ca1f879e2e37d4f43f5d0ea73b071222955))
* **eslint:** support template literal interpolation for define-style ([5be5080](https://github.com/vuejs/vue-jsx-vapor/commit/5be5080d47558c000f4d6ba39673ad93856fb2d8))
* **macros/volar:** allow define styles after the ReturnStatement ([255479b](https://github.com/vuejs/vue-jsx-vapor/commit/255479b00fc004474468fde554bbd7649066137a))
* **vue-jsx-vapor:** support h and Fragment ([c4e0bbe](https://github.com/vuejs/vue-jsx-vapor/commit/c4e0bbe114d903f2c9a8e240ed324bfd3b2bbc5a))



## [2.5.2](https://github.com/vuejs/vue-jsx-vapor/compare/v2.5.1...v2.5.2) (2025-07-17)


### Bug Fixes

* **ts-macro:** prevent useSelector undefined error ([1800b44](https://github.com/vuejs/vue-jsx-vapor/commit/1800b444bfeb3378151e1e66f842652a6c20b3e8))
* **vue-jsx-vapor:** generate correct HMR ([27179f6](https://github.com/vuejs/vue-jsx-vapor/commit/27179f6491bbcd2e64628e868a8d7090c8f07eb4))


### Features

* **macros:** support vue sfc ([0106c80](https://github.com/vuejs/vue-jsx-vapor/commit/0106c80a515851f7ec8e87d76999bc91a17ea4a7))
* **vue-jsx-vapor:** support vue sfc ([c24d3c7](https://github.com/vuejs/vue-jsx-vapor/commit/c24d3c73b6195b57c3b49ecd663ab3dfa0ae5e0a))



## [2.5.1](https://github.com/vuejs/vue-jsx-vapor/compare/v2.5.0...v2.5.1) (2025-07-13)


### Bug Fixes

* **compiler:** remove compiler error for v-slots ([033d62f](https://github.com/vuejs/vue-jsx-vapor/commit/033d62fcbc36ae2474792e6fe5f7a698495d00df))
* **compiler:** use underscores instead of colons for cached name ([0ffd1c6](https://github.com/vuejs/vue-jsx-vapor/commit/0ffd1c68339cdb1a935f91ee781b1ebabaf4c981))


### Features

* **compiler:** generate more efficient runtime code for specific interpolation patterns ([3c22e74](https://github.com/vuejs/vue-jsx-vapor/commit/3c22e749fca4a863fe3f0c004924ad972f6c00e3))
* **compiler:** support JSXNamespacedName for v-model ([941fb83](https://github.com/vuejs/vue-jsx-vapor/commit/941fb83933c8240bdaf8d8995b4a6a291b7965e9))
* **vue-jsx-vapor:** add getCurrentInstance ([067f018](https://github.com/vuejs/vue-jsx-vapor/commit/067f018817afcebb98ec6199dd1aba5ff7458609))



# [2.5.0](https://github.com/vuejs/vue-jsx-vapor/compare/v2.4.8...v2.5.0) (2025-07-07)


### Features

* add generator ([#9](https://github.com/vuejs/vue-jsx-vapor/issues/9)) ([88aea39](https://github.com/vuejs/vue-jsx-vapor/commit/88aea3989c8c99a6888901bfa91993736ff93683))



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



