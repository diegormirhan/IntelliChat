export function toggleModal(popupType) {
    const popupTheme = document.querySelector('.popup-theme');
    const popupSettings = document.querySelector('.popup-settings');

    if (popupType === 'theme') {
        if (popupTheme.style.display === 'none') {
            popupSettings.style.display = 'none';
            popupTheme.style.display = 'flex';
        } else {
            popupTheme.style.display = 'none';
        }
    } else if (popupType === 'settings') {
        if (popupSettings.style.display === 'none') {
            popupTheme.style.display = 'none';
            popupSettings.style.display = 'flex';
        } else {
            popupSettings.style.display = 'none';
        }
    }
}

window.toggleModal = toggleModal;