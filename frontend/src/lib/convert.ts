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
    static cnbody = "cnbody";
    static cnhead = "cnhead";

    basePage: string = "";
    constructor(private base: string, private getText: (url: string) => Promise<string>,
        private getBlob: (url: string) => Promise<Blob>) {
        this.basePage = base + "page/";
    }

    /**
     * The standard URL interface of the webbrowser is not made for the /page/domain setup
     * cybernode uses here.
     * So getBase needs to check multiple cases.
     * 
     * @param domain the first path after the /page
     * @param url the full url to convert
     * @returns a path that should be available in the storage
     */
    getBase(domain: string, url: string): string {
        if (url.startsWith(this.basePage + domain)) {
            return url.replace(this.basePage, "");
        }
        if (url.startsWith(this.basePage)) {
            return url.replace(this.basePage, domain + "/");
        }
        if (url.startsWith(this.base)){
            return url.replace(this.base, `${domain}/`)
        }
        return url;
    }

    async convert(url: string): Promise<HTMLElement> {
        const parser = new DOMParser();
        let domain = url.replace(this.base, "").replace(/\/.*/, "");
        const pageHtml = await this.getText(url);
        const docTmp = parser.parseFromString(pageHtml, 'text/html');

        const doc = parser.parseFromString("", 'text/html');
        await this.copyHead(domain, doc.head, docTmp.head.childNodes);
        await this.copyBody(domain, doc.body, docTmp.body.childNodes);

        return doc.documentElement;
    }

    async cleanBodyNode(domain: string, dst: Node, src: Node) {
        switch (src.nodeName) {
            case "IMG":
                const img = src as HTMLImageElement;
                if (img.src.startsWith(document.location.origin)) {
                    const imgSrc = this.getBase(domain, img.src);
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
                if (a.href.startsWith(this.base)) {
                    a.setAttribute('onclick', `event.preventDefault(); nextPage('${a.pathname}');`);
                }
                break;
        }
        dst.appendChild(src);
    }

    async copyBody(domain: string, dst: Node, src: NodeListOf<Node>) {
        for (let i = 0; i < src.length; i++) {
            const c = src[i];
            await this.cleanBodyNode(domain, dst, c.cloneNode(false));
            if (c.childNodes.length > 0) {
                await this.copyBody(domain, dst.lastChild!, c.childNodes);
            }
        }
    }

    async cleanHeadNode(domain: string, dst: Node, src: Node) {
        if (src.nodeName === "LINK") {
            const link = src as HTMLLinkElement;
            if (link.rel === "stylesheet") {
                console.log(`base: ${link.baseURI}, href: ${link.href}`);
                const cssText = await this.getText(this.getBase(domain, link.href));
                const styleElement = document.createElement('style');
                styleElement.textContent = cssText;
                styleElement.id = PreparePage.cnhead;
                const previousStyle = document.getElementById(PreparePage.cnhead);
                if (previousStyle !== null){
                    document.head.removeChild(previousStyle);
                }
                document.head.appendChild(styleElement);
                for (let i = 0; i < styleElement.sheet?.cssRules.length!; i++) {
                    const rule = styleElement.sheet!.cssRules[i] as CSSStyleRule;
                    rule.selectorText = `div#${PreparePage.cnbody} ${rule.selectorText}`;
                }
            }
        }
    }

    async copyHead(domain: string, dst: Node, src: NodeListOf<Node>) {
        for (let i = 0; i < src.length; i++) {
            const c = src[i];
            await this.cleanHeadNode(domain, dst, c.cloneNode(false));
            if (c.childNodes.length > 0) {
                await this.copyHead(domain, dst.lastChild!, c.childNodes);
            }
        }
    }
}