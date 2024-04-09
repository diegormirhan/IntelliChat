export function lightMode() {
    const body = document.body;
    body.classList.remove('dark-mode')
    body.classList.add('light-mode');
    localStorage.setItem("theme", "light-mode");
}

export function darkMode() {
    const body = document.body;
    body.classList.remove('light-mode')
    body.classList.add('dark-mode');
    localStorage.setItem("theme", "dark-mode");
}

export function systemMode() {
    const body = document.body;
    body.classList.remove('light-mode')
    body.classList.remove('dark-mode');
    localStorage.setItem("theme", "system-mode");
}

window.lightMode = lightMode;
window.darkMode = darkMode;
window.systemMode = systemMode