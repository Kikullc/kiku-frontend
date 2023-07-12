<script lang="ts">
    import { route } from '../../scripts/store.js';
    import {goto} from "$app/navigation";
    let currentRoute = '/';
    // Подписываемся на обновления маршрута
    route.subscribe(value => {
        currentRoute = value;
    });
    let token = localStorage.getItem('jwt');
    if(!token){
        goto('/')
    }
</script>
<div class="bg-[#1B1B1B] w-full h-[60px] flex items-center">
    <a class="align-middle text-center basis-1/6 {currentRoute === '/app' ? 'text-[#AE88FF]' : 'text-white'}  ml-[3%]" href="/app">Главная</a>

    <a class="align-middle text-center basis-1/6 {currentRoute === '/app/playlists' ? 'text-[#AE88FF]' : 'text-white'}" href="/app/playlists">Плейлисты</a>

    <a class="align-middle text-center basis-1/6 {currentRoute === '/app/favorites' ? 'text-[#AE88FF]' : 'text-white'}" href="/app/favorites">Любимое</a>

    <div class="flex justify-end basis-2/5 items-center">
        <svg class="mr-[1%]" width="12" height="14" viewBox="0 0 12 14" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M5.5 10C7.98528 10 10 7.98528 10 5.5C10 3.01472 7.98528 1 5.5 1C3.01472 1 1 3.01472 1 5.5C1 7.98528 3.01472 10 5.5 10Z" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M10.9091 12.0834L9.09097 9.91666" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <input type="text" class="rounded-[12px] bg-[#454545]">
    </div>
    <div class="flex justify-end basis-1/5 items-center">
        <a class="align-middle mr-[4%] {currentRoute === '/app/profile' ? 'text-[#AE88FF]' : 'text-white'}" href="/app/profile">
            neoko
        </a>
        <a class="mr-[5%]" href="/profile">
            <svg width="45" height="45" viewBox="0 0 45 45" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="22.5" cy="22.5" r="22.5" fill="#D9D9D9"/>
            </svg>
        </a>
    </div>
</div>
    <slot />