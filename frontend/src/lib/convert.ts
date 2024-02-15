/**
 * Pros: Built-in, lightweight, no external dependencies.
Cons: May require more manual DOM manipulation.
Parsing: const parser = new DOMParser(); const doc = parser.parseFromString(htmlString, 'text/html');.
Modification: Access/modify elements using DOM properties and methods.
Re-creation: const modifiedHtml = doc.body.outerHTML;.
Script Execution: evaluate('function(){/*your script here/}', doc, null, null, 'return')();. (Requires careful context-aware handling.)
 */
export function linkToScript(html: string, base: string): string {
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');
    const elements = doc.getElementsByTagName("*");
    for (let i = 0; i < elements.length; i++) {
        const el = elements[i];
        if (el instanceof HTMLAnchorElement){
            const href = el.href;
            const pageUrl = new URL(href);
            if (href.startsWith(`${base}/page`)){
                el.setAttribute('onclick', `event.preventDefault(); nextPage('${pageUrl.pathname}');`);
            }
        }
    }
    return doc.body.outerHTML;
}