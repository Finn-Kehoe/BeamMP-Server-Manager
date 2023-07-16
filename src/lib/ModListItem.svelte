<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Mod } from "./mod";

    export let modObject: Mod;
    let isActive = modObject.is_active;

    async function change_activation(internal_name: string) {
        await invoke("change_mod_activation", {
            internal_name: internal_name,
        }).catch((e) => {
            console.log("Error changing activation state for mod: ", e);
        });
    }

    $: isActive && change_activation(modObject.internal_name);
    
</script>

<li>
    <div class="main-body">
        <div class="details">
            <span class="internal-name">{modObject.internal_name}</span>
            <br />
            <span class="car-name">{modObject.details["brand"]} {modObject.details["name"]}</span>
        </div>
        <div class="action-buttons">
            <label class="on-off switch">
                <input type="checkbox" bind:checked={isActive}>
                <span class="slider round"></span>
            </label>
            <button class="delete button">
                <svg
                    style="fill: currentColor;"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 512 512">
                        <path
                            d="M199 103v50h-78v30h270v-30h-78v-50H199zm18 18h78v32h-78v-32zm-79.002 80l30.106 286h175.794l30.104-286H137.998zm62.338 13.38l.64 8.98 16 224 .643 8.976-17.956 1.283-.64-8.98-16-224-.643-8.976 17.956-1.283zm111.328 0l17.955 1.284-.643 8.977-16 224-.64 8.98-17.956-1.284.643-8.977 16-224 .64-8.98zM247 215h18v242h-18V215z"
                        />
                </svg>
                <!-- <img src="/trash-can.svg" alt="Delete" /> -->
            </button>
        </div>
    </div>
</li>

<style>
    .main-body {
        height: 100px;
        width: 300px;
        border-style: solid;
        border-width: medium;
        display: flex;
    }
    .details {
        padding: 0.6em;
    }
    .action-buttons {
        height: 30px;
        width: 30px;
    }
    .delete.button svg {
        color: white;
        height: 30px;
        width: 30px;
    }
    .delete.button:hover, .delete.button svg:hover {
        color: rgb(252, 77, 77);
    }
     /* The switch - the box around the slider */
    .switch {
    position: relative;
    display: inline-block;
    width: 50px;
    height: 24px;
    }

    /* Hide default HTML checkbox */
    .switch input {
    opacity: 0;
    width: 0;
    height: 0;
    }

    /* The slider */
    .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #1a1a1a;
    -webkit-transition: .4s;
    transition: .4s;
    }

    .slider:before {
    position: absolute;
    content: "";
    height: 20px;
    width: 20px;
    left: 2px;
    bottom: 2px;
    background-color: white;
    -webkit-transition: .4s;
    transition: .4s;
    }

    input:checked + .slider {
    background-color: #ff7722;
    }

    input:focus + .slider {
    box-shadow: 0 0 1px #ff7722;
    }

    input:checked + .slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
    }

    /* Rounded sliders */
    .slider.round {
    border-radius: 34px;
    }

    .slider.round:before {
    border-radius: 50%;
    } 
</style>
