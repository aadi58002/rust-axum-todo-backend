<script lang="ts">
  import Input from 'atoms/Input.svelte';
let username: string,password: string,email: string;
</script>

<div p-10 bg-gray-200 rounded-xl dark:bg-gray-800>
  <form class="flex flex-col">
    <!-- UserName input -->
    <Input type="text" placeholder="Username" bind:value={username}/>

    <!-- Email input -->
    <Input type="password" placeholder="Email address" bind:value={email}/>

    <!-- Password input -->
    <Input type="password" placeholder="Password" bind:value={password}/>
  </form>
  <div class="flex flex-col">
    <button w="1/2" mx-auto text-xl mb-5 bg-blue-400 border-0 rounded-md p-2 >Register</button>
    <div>
      already have a account?
      <a text-blue inline href="/signIn">Sign In</a>
    </div>
  </div>
</div>
