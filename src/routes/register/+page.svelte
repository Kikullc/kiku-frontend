<script>
    import {goto} from "$app/navigation";

    let username = '';
    let password = '';
    let email = '';

    const submitForm = async () => {
        const response = await fetch('https://kiku.social/api/auth/signup', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password, email }),
        });

        if (!response.ok) {
            const message = `Произошла ошибка: ${response.status}`;
            throw new Error(message);
        }

        const data = await response.json();
        localStorage.setItem('jwt', data.token);
        goto('/app')
    };
</script>

<div>
    <form on:submit|preventDefault={submitForm}>
        <input bind:value={username} type="text" placeholder="Логин" />
        <input bind:value={password} type="password" placeholder="Пароль" />
        <input bind:value={email} type="text" placeholder="Email" />
        <button type="submit">Регистрация</button>
    </form>
    <p>Есть аккаунт? <a href="/">Авторизация</a> </p>
</div>
