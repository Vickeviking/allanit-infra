# Allanit Dashboard - Mock/Real Data Switch

## 🎛️ Enkel Switch-konfiguration

### Environment Variables

**Development (.env)**
```bash
# Mock data mode
VITE_USE_MOCK=true
VITE_API_URL=http://localhost:8000
```

**Production (.env)**
```bash
# Real API mode
VITE_USE_MOCK=false
VITE_API_URL=https://api.allanit.com
```

---

## 🔄 Switch Implementation

### Frontend Architecture
- **Single Environment Variable**: `VITE_USE_MOCK` controls everything
- **Unified HTTP Client**: Same interface for mock and real data
- **Zero Code Changes**: Frontend code works identically with both modes
- **Automatic Detection**: Console logs show current mode

### Mock Client Features
- **Static Data**: Returns predefined mock data
- **Network Simulation**: Simulates real API delays
- **Console Logging**: Shows all mock operations
- **Full Coverage**: All endpoints have mock implementations

### Real API Features
- **JWT Authentication**: Automatic token handling
- **Error Handling**: 401 redirects to login
- **Request Interceptors**: Automatic auth headers
- **Response Interceptors**: Error handling and token refresh

---

## 🚀 Development Workflow

### Phase 1: Mock Development
1. **Frontend Development**: Build UI with mock data
2. **User Testing**: Test workflows with realistic data
3. **UI/UX Refinement**: Perfect user experience
4. **No Backend Dependencies**: Frontend can be developed independently

### Phase 2: Backend Development
1. **Parallel Development**: Backend developed alongside frontend
2. **API Implementation**: Implement endpoints one by one
3. **Database Setup**: PostgreSQL schema and data
4. **Testing**: Backend testing with mock frontend

### Phase 3: Integration
1. **Switch to Real API**: Change environment variable
2. **End-to-End Testing**: Full system testing
3. **Performance Optimization**: Real-world performance tuning
4. **Production Deployment**: Go live with real data

---

## 🎯 Benefits

### ✅ **Zero Code Changes**
- Same frontend code works with both mock and real data
- No conditional logic scattered throughout the app
- Clean separation of concerns

### ✅ **Easy Testing**
- Test with mock data during development
- Switch to real API for integration testing
- No need to mock individual API calls

### ✅ **Gradual Migration**
- Start with mock data
- Implement backend endpoints one by one
- Switch to real API when ready

### ✅ **Development Speed**
- Frontend development can start immediately
- No waiting for backend implementation
- Parallel development of frontend and backend

---

## 🛠️ Quick Commands

```bash
# Mock mode (default)
npm run dev

# Real API mode
VITE_USE_MOCK=false npm run dev

# Build for production
VITE_USE_MOCK=false npm run build
```

---

## 🔍 Debugging Tips

### Mock Mode Debugging
- **Console Logs**: All mock operations are logged
- **Data Inspection**: Mock data is easily accessible
- **Network Simulation**: Adjustable delays for testing
- **Error Simulation**: Mock error responses for testing

### Real API Debugging
- **Network Tab**: Monitor real API calls
- **Authentication**: Check JWT token validity
- **Error Responses**: Real error handling
- **Performance**: Real-world performance metrics

---

## 📊 Performance Comparison

### Mock Mode
- **Response Time**: ~100ms simulated delay
- **Data Volume**: Limited to mock data size
- **Network**: No real network calls
- **Authentication**: Simulated login

### Real API Mode
- **Response Time**: Real network latency
- **Data Volume**: Full database queries
- **Network**: Real HTTP requests
- **Authentication**: JWT token validation

---

## 🚀 Migration Checklist

### Pre-Migration
- [ ] Frontend fully developed with mock data
- [ ] All user workflows tested
- [ ] UI/UX finalized
- [ ] Backend API endpoints implemented
- [ ] Database schema created
- [ ] Authentication system ready

### Migration Day
- [ ] Change `VITE_USE_MOCK=false`
- [ ] Deploy backend to production
- [ ] Update frontend environment variables
- [ ] Deploy frontend to production
- [ ] Test all critical workflows
- [ ] Monitor system performance

### Post-Migration
- [ ] Monitor error logs
- [ ] Check performance metrics
- [ ] User feedback collection
- [ ] System optimization
- [ ] Documentation updates

---

*Denna switch-mekanism gör migrationen från mock-data till riktig backend enkelt och riskfritt.*