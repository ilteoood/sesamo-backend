import DocumentData = FirebaseFirestore.DocumentData;
import QueryDocumentSnapshot = FirebaseFirestore.QueryDocumentSnapshot;

export class FirebaseServer {
    static readonly FIELDS = ["name", "type"];

    name: string;
    type: string;
    allowedDevices: string[] = [];
    actions: Map<string, Map<string, string>> = new Map();

    static async convertServerDocument(documentData: DocumentData): Promise<FirebaseServer> {
        const documentContent = documentData.data();
        const configurationDocument = await documentData.ref.collection("configurations").get();
        const firebaseServer = new FirebaseServer();
        this.FIELDS.forEach(field => firebaseServer[field] = documentContent[field]);
        firebaseServer.allowedDevices = this.findAllowedDevices(configurationDocument.docs);
        firebaseServer.actions = this.createActionsMap(configurationDocument.docs);
        return firebaseServer;
    }

    static findAllowedDevices(documents: QueryDocumentSnapshot[]) {
        return documents.find(this.isAllowedDevices).get('list');
    }

    static isAllowedDevices(document: QueryDocumentSnapshot) {
        return document.id === "allowedDevices";
    }

    static createActionsMap(documents: QueryDocumentSnapshot[]) {
        const actions = new Map();
        documents
            .filter(document => !this.isAllowedDevices(document))
            .forEach(document => actions[document.id] = document.data());
        return actions;
    }
}
