<div class="container">
    <div>

        <Textfield
            variant="filled"
            bind:value={signUp.first_name}
            label="First Name"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isFirstNameValid}
        >
            <Icon class="material-icons" slot="leadingIcon">person</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={signUp.last_name}
            label="Last Name"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isLastNameValid}
        >
            <Icon class="material-icons" slot="leadingIcon">person</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={signUp.email}
            label="Email"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isEmailValid}
        >
            <Icon class="material-icons" slot="leadingIcon">email</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={signUp.password}
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
            on:click={handleSignUp}
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
    import authStore from '../store/auth.store';
    import notifStore from '../store/notification.store';
    import { axiosPost } from '../utils/api.utils';
    import type { ISignUp } from '../store/auth.store';
    import type { IUser } from '../types/models';

    const signUp: ISignUp = {
        first_name: "",
        last_name: "",
        email: "",
        password: "",
    }

    $: isFirstNameValid = signUp.first_name.trim().length > 0;
    $: isLastNameValid = signUp.last_name.trim().length > 0;
    $: isPasswordValid = signUp.password.trim().length >= 8;
    $: isEmailValid = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(signUp.email);
    $: isButtonDisabled = !isFirstNameValid || !isLastNameValid || !isPasswordValid || !isEmailValid;

    const handleSignUp = async () => {
        const response = await axiosPost<IUser, ISignUp>("/api/auth/signup", signUp);

        if (response.error || !response.data) {
            console.log(response.error);
            $notifStore.open(response.error ?? 'Error Logging in', 'error');
            return;
        }

        $authStore = response.data.content as IUser;

        $notifStore.open('Successfully logged in', 'success');
    };
</script>