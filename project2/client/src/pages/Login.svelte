<div class="container">
    <form on:submit|preventDefault={submit}>
        <Textfield
            variant="filled"
            bind:value={$signIn.email}
            label="Email"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isEmailValid}
        >
            <Icon class="material-icons" slot="leadingIcon">email</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={$signIn.password}
            label="Password"
            type="password"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isPasswordValid}
        >
            <Icon class="material-icons" slot="leadingIcon">key</Icon>
        </Textfield>

        <Button
            disabled={isButtonDisabled}
            color="secondary"
            class="button"
            variant="raised"
            type="submit"
            style="width: 100%;"
            >
            <Label>Log In</Label>
        </Button>
    </form>
</div>

<style>
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  }

</style>

<script lang="ts">
    import Button, { Label } from '@smui/button';
    import Textfield from '@smui/textfield';
    import Icon from '@smui/textfield/icon';
    import { signIn } from '../store/auth.store';

    $: isPasswordValid = $signIn.password.trim().length > 0;
    $: isEmailValid = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test($signIn.email);
    $: isButtonDisabled = !isPasswordValid || !isEmailValid;

    const submit = async () => {
        console.log($signIn);
    };
</script>