export function throttle(delay: number, func: Function) {
    let prev = 0;
    return function (this: any) {
        const context = this;
        const args = arguments;
        const curr = Date.now()
        if (curr - prev > delay) {
            func.apply(context, args)
            prev = curr
        }
    }
}

export function getCache<T>(name: string, defaults: T): T {
    const cache = localStorage.getItem(name)
    if (cache) {
        return JSON.parse(cache)
    } else {
        return defaults
    }
}

export function setCache<T>(name: string, value: T) {
    const jsonString = JSON.stringify(value)
    localStorage.setItem(name, jsonString)
}