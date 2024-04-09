export function themePopup() {
    const popupTheme = document.querySelector('.popup-theme');
    const modal = document.getElementById('theme');

    popupTheme.addEventListener('click', (event) => {
        event.stopPropagation();
    })

    // Verify if the user clicked outside the popup theme
    document.addEventListener('click', (event) => {
        if (event.target !== modal) popupTheme.style.display = 'none';
    })
}

window.themePopup = themePopup