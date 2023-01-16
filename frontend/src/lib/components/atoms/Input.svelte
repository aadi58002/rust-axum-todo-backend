<script lang="ts">
  type HTMLInputTypeAttribute = 'password' | 'text' | 'email';
  let Class: string = '';
  export let type: HTMLInputTypeAttribute, placeholder: string, value: string;
  export { Class as class };
  const onInput = (e: InputEvent) => (value = (e.target as HTMLInputElement).value);
</script>

<input
  {type}
  block
  w-full
  px-4
  mb-6
  py-2
  text-xl
  font-normal
  text-gray-700
  bg-white
  bg-clip-padding
  border
  border-solid
  border-gray-300
  rounded
  transition
  ease-in-out
  m-0
  focus:text-gray-700
  focus:bg-white
  focus:border-blue-600
  focus:outline-none
  class={Class}
  {placeholder}
  {value}
  on:input={onInput}
/>
