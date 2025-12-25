# **Build *Tinybase*: A PocketBase Alternative Using Rust + libSQL + React Admin UI**

**Do not ask questions.
Do not wait for clarification.
Complete the entire design and publish all deliverables.**

---

# **🔥 PROJECT NAME: Tinybase**

Build a complete backend platform called **Tinybase** — a modern, lightweight PocketBase alternative implemented with:

* **Rust backend**
* **libSQL** as the embedded/remote sync database
* **React Admin Dashboard** with CRUD, schema builder, settings UI
* Real-time API
* File storage
* Access rules
* Hooks
* Migrations
* Dynamic collections system

This should feel like a clean, modern, developer-friendly version of PocketBase.

---

# **1. Tinybase Backend (Rust + libSQL)**

### **Tech Stack**

* Rust (stable)
* axum or actix-web
* libSQL client
* serde / serde_json
* JWT (jsonwebtoken)
* argon2 or bcrypt
* tokio
* tower middlewares

### **Backend Feature Set**

Implement a full backend with:

#### **A. Authentication**

* Email + password
* Admin accounts
* JWT-based sessions

#### **B. Collections**

Dynamic collections system like PocketBase:

Users must be able to create collections with fields such as:

* text
* number
* bool
* date
* file
* JSON
* relation
* array

With field metadata:

* required
* unique
* default
* min/max
* regex (for text)

#### **C. CRUD API**

* Auto-generated CRUD endpoints for any collection
* Pagination
* Filtering
* Relations expansion (`?expand=author,comments.user`)

#### **D. Real-time API**

* WebSocket endpoint: `/api/realtime`
* Publish events for:

  * create
  * update
  * delete

#### **E. File Storage**

* Local filesystem
* S3-compatible drivers (MinIO, Backblaze, DigitalOcean, etc.)

#### **F. Rules Engine**

* Read access rules
* Write access rules
* Update rules
* Delete rules

Use a rule-expression language similar to PocketBase’s.

#### **G. Hooks**

Pre/post hooks:

* beforeCreate
* afterCreate
* beforeUpdate
* afterUpdate
* beforeDelete
* afterDelete

#### **H. Migrations**

Built-in migrations engine:

* automatic schema diffs
* versioned migrations folder
* CLI commands

#### **I. Admin & System APIs**

* System stats
* Activity logs
* Settings API
* File storage settings
* SMTP settings

---

# **2. Tinybase Admin Dashboard (React)**

Build a complete React-based admin UI for Tinybase.

### **Stack**

* React + TypeScript
* Vite or Next.js
* TailwindCSS
* ShadCN/UI
* React Query
* Zustand
* Monaco Editor for rule expressions (optional)

---

# **Admin Dashboard Features**

### **A. Login Page**

* Admin login
* JWT stored in httpOnly cookies

---

### **B. Collections Manager**

Exactly like PocketBase:

* Create collection
* Edit collection
* Delete collection
* Add fields
* Remove fields
* Edit field attributes
* Reorder fields
* Configure rules (view, create, update, delete)

UI should include:

* Field type selector
* Field settings drawer
* Validation helpers

---

### **C. Records Manager**

For any collection:

* Paginated table
* Search
* Filters
* Create record modal
* Edit record modal
* Delete record
* File upload fields
* Relation selectors
* JSON editor for JSON fields

---

### **D. Real-time Viewer**

Show:

* Record updates as they happen
* WebSocket status
* Event logs

---

### **E. Settings Pages**

Implement screens for:

* General settings
* SMTP
* File storage
* API keys
* Server info
* Database sync settings
* Logs viewer

---

### **F. Dashboard Home**

Display:

* Total collections
* Total records
* Total users
* Recent activity
* System health metrics

---

# **3. Developer Deliverables**

Generate:

### ✔ Full system architecture

### ✔ Folder structure (backend + frontend)

### ✔ Database schema (collections, fields, relations)

### ✔ REST + WebSocket API documentation

### ✔ React UI structure and components

### ✔ Example Rust code for all key modules

### ✔ Example API handlers

### ✔ Example migrations

### ✔ Example hooks

### ✔ Admin dashboard mockups or component layout

### ✔ CLI tool design:

* `tinybase serve`
* `tinybase migrate`
* `tinybase collections create`
* `tinybase admin create`

---

# **4. Output Format**

Deliver everything in **Markdown**, including:

* Code blocks
* Schemas
* Architecture diagrams (ASCII is fine)
* API references
* UI layout
* Folder trees

---

# **5. Final Rule**

**Do not ask any questions.
Do not stop until Tinybase is fully planned and documented.
Publish the complete solution.**
