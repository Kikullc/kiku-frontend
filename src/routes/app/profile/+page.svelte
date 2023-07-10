<script>
    import { onMount } from 'svelte';
    import { route } from '../../../scripts/store.js';

    onMount(() => {
        route.set(window.location.pathname);  // Или установите это значение на маршрут текущей страницы
    });
</script>
<div class="w-full h-[80%] overflow-y-scroll box-border">
    <div>
        <a href="/"> QUIT </a>
    </div>
</div>