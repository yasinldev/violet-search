const input = document.getElementById('search-bar');
const dropdown = document.querySelector(".input-dropdown");
const inputGroup = input.parentElement;

input.addEventListener('focus', () => {
    inputGroup.classList.add('input-focused');
    dropdown.classList.add('dropdown-active');
});

input.addEventListener('input', () => {
    const inputLength = input.value.length;
    if (inputLength <= 3 || inputLength === 0) {
        dropdown.classList.remove('dropdown-active');
    } else if (inputLength > 3) {
        dropdown.classList.add('dropdown-active');
    }
});

input.addEventListener('blur', () => {
    inputGroup.classList.remove('input-focused');
    dropdown.classList.remove("dropdown-active");
});

const sidebarId = document.getElementById('sidebar');
const sidebar = document.querySelector(".sidebar");
sidebarId.addEventListener('click', () => {
    sidebar.classList.toggle('sidebar-active');
});

sidebarId.addEventListener('blur', () => {
    sidebar.classList.remove('sidebar-active');
});


const close_modal = document.getElementById('modal-close');

const themes_and_options = document.getElementById('themes_and_options');
const themes_modal = document.getElementById('themes_modal');

themes_and_options.addEventListener('click', () => {
    themes_modal.classList.add('modal-active');
});

const language_and_results = document.getElementById('language_and_results');
const language_modal = document.getElementById('language_modal');

const privacy_policy = document.getElementById('privacy_policy');
const privacy_modal = document.getElementById('privacy_modal');

const terms_of_service = document.getElementById('terms_of_service');
const terms_modal = document.getElementById('terms_modal');

const developers = document.getElementById('developers');
const developers_modal = document.getElementById('developers_modal');


close_modal.addEventListener('click', () => {
    themes_modal.classList.remove('modal-active');
    language_modal.classList.remove('modal-active');
    privacy_modal.classList.remove('modal-active');
    terms_modal.classList.remove('modal-active');
    developers_modal.classList.remove('modal-active');
});

const save_wallpaper = document.getElementById('save_wallpaper');
const reset_wallpaper = document.getElementById('reset_wallpaper');
const wallpaper_input = document.getElementById('wallpaper_input');

save_wallpaper.addEventListener('click', function() {
    localStorage.setItem('wallpaper', wallpaper_input.value);
    location.reload();
});

document.body.style.backgroundImage = `url(${localStorage.getItem('wallpaper')})`;
document.body.style.backgroundRepeat = 'no-repeat';
document.body.style.backgroundAttachment = 'fixed';
document.body.style.backgroundPosition = 'center';
document.body.style.backgroundSize = 'cover';
save_wallpaper.placeholder = localStorage.getItem('wallpaper');

reset_wallpaper.addEventListener('click', function() {
    localStorage.removeItem('wallpaper');
    location.reload();
});

const ace_save = document.getElementById('save_editor');
const ace_reset = document.getElementById('reset_editor');
const styled_editor = document.getElementById('styled_editor');
const editor_text = document.getElementById('editor_text');

const editor = ace.edit("editor");
editor.setTheme("ace/theme/monokai");
editor.session.setMode("ace/mode/css");
ace_save.addEventListener('click', function() {
    localStorage.setItem('editor', `<style>${editor.getValue()}</style>`);
    styled_editor.innerHTML = localStorage.getItem('editor');
    editor_text.innerHTML = localStorage.getItem('editor');
    location.reload();
});

styled_editor.innerHTML = localStorage.getItem('editor');
editor_text.innerHTML = localStorage.getItem('editor');

ace_reset.addEventListener('click', function() {
    localStorage.removeItem('editor');
    styled_editor.innerHTML = '';
    editor_text.value = '/* Enter your CSS code here */';
    location.reload();
});
