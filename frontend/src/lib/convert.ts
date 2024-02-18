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
        if (el instanceof HTMLAnchorElement) {
            const href = el.href;
            const pageUrl = new URL(href);
            if (href.startsWith(`${base}/page`)) {
                el.setAttribute('onclick', `event.preventDefault(); nextPage('${pageUrl.pathname}');`);
            }
        }
    }
    return doc.body.outerHTML;
}

export class PreparePage {
    constructor(private getText: (url: string) => Promise<string>,
        private getBlob: (url: string) => Promise<Blob>) { }

    async convert(url: string): Promise<HTMLElement> {
        const parser = new DOMParser();
        const pageHtml = await this.getText(url);
        const docTmp = parser.parseFromString(pageHtml, 'text/html');

        const doc = parser.parseFromString("", 'text/html');
        await this.copyHead(doc.head, docTmp.head.childNodes);
        await this.copyBody(doc.body, docTmp.body.childNodes);

        return doc.documentElement;
    }

    async cleanBodyNode(dst: Node, src: Node) {
        switch (src.nodeName) {
            case "IMG":
                const img = src as HTMLImageElement;
                if (img.src.startsWith(document.location.origin)) {
                    const imgSrc = img.src.replace(document.location.origin, "");
                    img.src = "";
                    img.alt = "something";
                    const imgData = await this.getBlob(imgSrc);
                    var reader = new FileReader();
                    reader.readAsDataURL(imgData);
                    reader.onloadend = function () {
                        img.src = reader.result?.toString()!;
                    }
                }
                break;
            case "A":
                const a = src as HTMLAnchorElement;
                if (a.href.startsWith(document.location.origin)) {
                    a.setAttribute('onclick', `event.preventDefault(); nextPage('${a.pathname}');`);
                }
                break;
        }
        dst.appendChild(src);
    }

    async copyBody(dst: Node, src: NodeListOf<Node>) {
        for (let i = 0; i < src.length; i++) {
            const c = src[i];
            await this.cleanBodyNode(dst, c.cloneNode(false));
            if (c.childNodes.length > 0) {
                await this.copyBody(dst.lastChild!, c.childNodes);
            }
        }
    }

    async cleanHeadNode(dst: Node, src: Node) {
        if (src.nodeName === "LINK") {
            const link = src as HTMLLinkElement;
            if (link.rel === "stylesheet") {
                const cssText = await (await fetch(link.href)).text();
                const styleElement = document.createElement('style');
                styleElement.textContent = cssText;
                document.head.appendChild(styleElement);
                for (let i = 0; i < styleElement.sheet?.cssRules.length!; i++) {
                    const rule = styleElement.sheet!.cssRules[i] as CSSStyleRule;
                    rule.selectorText = `div#obj ${rule.selectorText}`;
                }
            }
        } else {
            dst.appendChild(src);
        }
        return true;
    }

    async copyHead(dst: Node, src: NodeListOf<Node>) {
        for (let i = 0; i < src.length; i++) {
            const c = src[i];
            await this.cleanHeadNode(dst, c.cloneNode(false));
            if (c.childNodes.length > 0) {
                await this.copyHead(dst.lastChild!, c.childNodes);
            }
        }
    }
}