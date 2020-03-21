import DocumentData = FirebaseFirestore.DocumentData;

export class FirebaseServer {
    static readonly FIELDS = ["allowedDevices", "name", "type"];

    allowedDevices: string[];
    name: string;
    type: string;

    static convertDocument(documentData: DocumentData): FirebaseServer {
        const documentContent = documentData.data();
        const firebaseServer = new FirebaseServer();
        this.FIELDS.forEach(field => firebaseServer[field] = documentContent[field]);
        return firebaseServer;
    }
}
