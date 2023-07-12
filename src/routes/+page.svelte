<script>
    import {goto} from "$app/navigation";

    let username = '';
    let password = '';

    const submitForm = async () => {
        const response = await fetch('https://kiku.social/api/auth/signin', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password }),
        });

        if (!response.ok) {
            const message = `Произошла ошибка: ${response.status}`;
            throw new Error(message);
        }

        const data = await response.json();
        console.log(data);
        localStorage.setItem('jwt', data.token);
        goto('/app')
    };
</script>

<div>
    <form on:submit|preventDefault={submitForm}>
        <input bind:value={username} type="text" placeholder="Логин" />
        <input bind:value={password} type="password" placeholder="Пароль" />
        <button type="submit">Авторизация</button>
    </form>
    <p>Нету аккаунта? <a href="/register">Регистрация</a> </p>
</div>
