<script lang="ts">
  import Input from 'atoms/Input.svelte';
  let username: string = '',
    password: string = '';
  const signIn = () => {
    console.log(username, password);
  };
</script>

<div p-10 bg-gray-200 rounded-xl dark:bg-gray-800>
  <form flex flex-col>
    <!-- Username input -->
    <Input type="text" placeholder="Username" bind:value={username} />

    <!-- Password input -->
    <Input type="password" placeholder="Password" bind:value={password} />
  </form>

  <div flex flex-col>
    <button
      w="1/2"
      mx-auto
      text-xl
      mb-5
      bg-green
      border-0
      rounded-md
      p-2
      on:click={signIn}
      on:keydown={signIn}>Sign In</button
    >
    <div>
      Don't have a account?
      <a text-blue inline href="/Register">Register</a>
    </div>
  </div>
</div>
