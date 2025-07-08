async function delay(seconds: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(function () {
      resolve()
    }, seconds * 1000)
  })
}

export { delay }
