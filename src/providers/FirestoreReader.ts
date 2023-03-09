import {Firestore} from '@google-cloud/firestore'
import {Injectable} from '@nestjs/common'

import {FirebaseServer} from '../models/firebase/FirebaseServer'
import QueryDocumentSnapshot = FirebaseFirestore.QueryDocumentSnapshot;

@Injectable()
export class FirestoreReader {

    private fireStoreClient = new Firestore()
    private servers: Map<string, FirebaseServer> = new Map()


    constructor() {
        this.fireStoreClient
            .collection('servers')
            .onSnapshot(async serversSnapshot => {
                this.servers.clear()
                serversSnapshot.docs
                    .forEach(await this.createServerEntry.bind(this))
            })
    }

    private async createServerEntry(documentSnapshot: QueryDocumentSnapshot) {
        this.servers[documentSnapshot.id] = await FirebaseServer.convertServerDocument(documentSnapshot)
    }

    public findServer(serverId: string): undefined | FirebaseServer {
        return this.servers[serverId]
    }

    public findConfigurations(serverId: string, object: string): undefined | Map<string, string> {
        const server = this.findServer(serverId)
        return server.actions[object]
    }

    public findAllowedDevices(serverId: string): string[] {
        return this.findServer(serverId).allowedDevices
    }

}
