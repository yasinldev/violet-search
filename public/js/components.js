// If focused #search-bar element show dropdowns

const input = document.getElementById('search-bar');
const dropdown = document.querySelector(".input-dropdown");
const inputGroup = input.parentElement;

input.addEventListener('focus', () => {
    inputGroup.classList.add('input-focused');
    dropdown.classList.add('dropdown-active');
});

// Getting searchbar lenght and adding class to dropdown
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

const loginId = document.getElementById("login");
const loginClose = document.getElementById("modal-close");
const loginModal = document.querySelector(".modal");

loginId.addEventListener('click', () => {
    loginModal.classList.toggle("modal-active");
    alert.classList.add("alert-show");
    alertPrimary.classList.add("alert-show");
});

loginClose.addEventListener('click', () => {
    loginModal.classList.remove("modal-active");
})

const closeAlert = document.getElementById("alert-close");
const alertPrimary = document.querySelector(".alert-primary");

const alertWarning = document.getElementById('alert-close-warning');
const alert = document.querySelector(".alert-warning");
closeAlert.addEventListener('click', () => {
    alertPrimary.classList.add("alert-remove");
    alertPrimary.classList.remove("alert-show");
});

alertWarning.addEventListener('click', () => {
    alert.classList.add("alert-remove");
    alert.classList.remove("alert-show");
});
