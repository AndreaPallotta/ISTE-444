<div class="container">
    <div>
        <Textfield
            variant="filled"
            bind:value={signIn.email}
            label="Email"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isEmailValid}
        >
            <Icon class="material-icons" slot="leadingIcon">email</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={signIn.password}
            label="Password"
            type="password"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isPasswordValid}
        >
            <Icon class="material-icons" slot="leadingIcon">key</Icon>
        </Textfield>

        <div class="mdc-typography--headline4" style="padding-bottom: 1rem;">
            Don't have an account?
            <Link to="/signup">Sign up here</Link>
        </div>

        <Button
            disabled={isButtonDisabled}
            color="primary"
            class="button"
            variant="raised"
            type="submit"
            style="width: 100%;"
            on:click={handleLogin}
            >
            <Label>Log In</Label>
        </Button>
    </div>
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
    import { Link } from 'svelte-navigator';
    import type { ISignIn, IUser } from '../store/auth.store';
    import authStore from '../store/auth.store';
    import notifStore from '../store/notification.store';
    import { axiosPost } from '../utils/api.utils';

    const signIn: ISignIn = {
        email: "",
        password: "",
    }

    $: isPasswordValid = signIn.password.trim().length >= 8;
    $: isEmailValid = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(signIn.email);
    $: isButtonDisabled = !isPasswordValid || !isEmailValid;

    const handleLogin = async () => {
        const response = await axiosPost<IUser, ISignIn>("/api/auth/login", signIn);

        if (response.error || !response.data) {
            $notifStore.open(response.error ?? 'Error Logging in', 'error');
            return;
        }

        $authStore = response.data.content as IUser;

        if ($authStore != null) {
            $notifStore.open('Successfully logged in', 'success');
        }
    };
</script>