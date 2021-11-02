function () {
  "use asm";

  function fibonacci (n) {
    n = n | 0;
    if (n < 2) {
      return 1
    }
    return (fibonacci(n - 2) | 0) + (fibonacci(n - 1) | 0) | 0
  }

  return {
    fibonacci: fibonacci,
  }
}