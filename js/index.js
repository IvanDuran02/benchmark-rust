const count = document.getElementById("count");

for (let i = 1; i <= 100000; i++) {
  setTimeout(() => {
    count.textContent = i;
  }, 250);
}
