import { invoke } from '@tauri-apps/api/tauri'

export function settingsPopup() {
    const popupSettings = document.querySelector(".popup-settings");
    const modal = document.getElementById("settings");

    popupSettings.addEventListener("click", (event) => {
        event.stopPropagation();
    });

    // Verify if the user clicked outside the popup settings
    document.addEventListener("click", (event) => {
        if (event.target !== modal) popupSettings.style.display = "none";
    });
}

export async function settingsSave() {
    const settingsValue = document.getElementById("api-key").value;
    localStorage.setItem("api-key", settingsValue);

    const savedApiKey = localStorage.getItem("api-key");
    console.log(savedApiKey)
    
    const response = await invoke('update_api_key', { apiKey: savedApiKey });
    console.log(response)
}

export function toggleApiKeyVisibility() {
    const apiKeyInput = document.getElementById("api-key");
    const showKey = document.getElementById("show-key");
    const showKeyLabel = document.getElementById("show-key-label");
    if (showKey.checked) {
        apiKeyInput.type = "text";
        showKeyLabel.textContent = "Hide key";
    } else {
        apiKeyInput.type = "password";
        showKeyLabel.textContent = "Show key";
    }
}


window.settingsPopup = settingsPopup;
window.settingsSave = settingsSave;
window.toggleApiKeyVisibility = toggleApiKeyVisibility;
