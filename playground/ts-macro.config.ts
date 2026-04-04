import vueJsxVapor from '../packages/vue-jsx-vapor/src/volar'

export default {
  exclude: ['**/slots.tsx'],
  plugins: [
    vueJsxVapor({
      macros: true,
    }),
  ],
}
